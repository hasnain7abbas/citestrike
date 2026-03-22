export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.svg","robots.txt"]),
	mimeTypes: {".svg":"image/svg+xml",".txt":"text/plain"},
	_: {
		client: {start:"_app/immutable/entry/start.B7a-dPQd.js",app:"_app/immutable/entry/app.Cv_-NHW7.js",imports:["_app/immutable/entry/start.B7a-dPQd.js","_app/immutable/chunks/_l7E0O-9.js","_app/immutable/chunks/DQ32N8F_.js","_app/immutable/chunks/DwETioo3.js","_app/immutable/entry/app.Cv_-NHW7.js","_app/immutable/chunks/DQ32N8F_.js","_app/immutable/chunks/DzWuZmQg.js","_app/immutable/chunks/LEypfyLm.js","_app/immutable/chunks/DwETioo3.js","_app/immutable/chunks/DvHuIOgm.js","_app/immutable/chunks/DCoxEa5e.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js'))
		],
		remotes: {
			
		},
		routes: [
			
		],
		prerendered_routes: new Set(["/"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
