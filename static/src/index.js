var indexPage = require('fishfront/ui/indexPage');
var dialog = require('fishfront/ui/dialog');
var $ = require('fishfront/ui/global');
indexPage.use({
	title:'Fish个人管理系统',
	init:function(){
		$.get('/login/islogin?t='+Math.random(),{},function(data){
			data = $.JSON.parse(data);
			if( data.code != 0 ){
				location.href = 'login.html';
				return;
			}
		});
	},
	logout:function(){
		$.get('/login/checkout?t='+Math.random(),{},function(data){
			data = $.JSON.parse(data);
			if( data.code != 0 ){
				dialog.message(data.msg);
				return;
			}
			location.href = 'login.html';
		});
	},
	menu:{
		'系统管理':{
			'系统管理':[
				{name:'帐号管理',url:'view/user/index.html'},
				{name:'密码管理',url:'view/password/index.html'}
			],
		},
		'财务管理':{
			'财务管理':[
				{name:'分类管理',url:'view/category/index.html'},
				{name:'银卡管理',url:'view/card/index.html'},
				{name:'账务管理',url:'view/account/index.html'},
				{name:'收支统计',url:'view/account/inoutStatistic.html'},
				{name:'结余统计',url:'view/account/totalStatistic.html'},
			],
		},
		'便捷工具':{
			'博客同步':[
				{name:'自动同步',url:'view/blog/indexAuto.html'},
				{name:'一键同步',url:'view/blog/index.html'},
			],
			'市一产科挂号':[
				{name:'自动挂号',url:'view/register/index.html'},
			],
			'便捷工具':[
				{name:'优惠卷工具',url:'view/coupon/view.html'},
			]
		},
	}
});
