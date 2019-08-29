use actix_web::{web, App, Error,HttpRequest, HttpServer, Responder,HttpResponse,error};
use actix_session::{CookieSession, Session};
use actix_web::middleware::Logger;
use listenfd::ListenFd;
use env_logger;
use futures::future::{ok, Future};

extern crate mysql_async as mysql;
use mysql::prelude::*;
use mysql::Pool;

fn index1(pool:web::Data<Pool>,session:Session)->impl Future<Item=String,Error=Error>{
	let conn = pool.get_conn();

	conn.and_then(|conn|{
		conn.query("select categoryId,userId,name,remark from t_category")
	}).and_then(|data|{
		data.collect::<(u32,String,String,String)>()
	}).map(move|(_,data)|{
	    let visit_count = if let Some(count) = session.get::<i32>("counter").unwrap() {
	        session.set("counter", count + 1).unwrap();
	        count+1
	    } else {
	        session.set("counter", 1).unwrap();
	        1
	    };
	    format!("{:?},visit_count:{}",data,visit_count)
	}).map_err(|e|{
		error::ErrorInternalServerError(e)
	})
}

fn main() {
	let database_url = "mysql://root:1@localhost:3306/FishMoney";
	let pool = mysql::Pool::new(database_url);

	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move|| {
        App::new()
        	.wrap(Logger::default())
        	.wrap(CookieSession::signed(&[0; 32]).secure(false))
        	.data(pool.clone())
            .route("/",web::get().to_async(index1))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8088").unwrap()
    };

    server.run().unwrap();
}