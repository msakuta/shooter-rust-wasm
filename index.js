/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".index.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./pkg/index_bg.wasm": function() {
/******/ 			return {
/******/ 				"./index_bg.js": {
/******/ 					"__wbg_log_4ff751541715bb7f": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_log_4ff751541715bb7f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_jsval_eq": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_jsval_eq"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_49f532f06a9786ee": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_Window_49f532f06a9786ee"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_c0366b39e4f4c89a": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_document_c0366b39e4f4c89a"](p0i32);
/******/ 					},
/******/ 					"__wbg_getElementById_15aef17a620252b4": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getElementById_15aef17a620252b4"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setinnerHTML_79084edd97462c07": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setinnerHTML_79084edd97462c07"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_WebGlRenderingContext_ef4e51c6e4133d85": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_WebGlRenderingContext_ef4e51c6e4133d85"](p0i32);
/******/ 					},
/******/ 					"__wbg_bufferData_dc5899657e9f1803": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_bufferData_dc5899657e9f1803"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_texImage2D_a4011abffe0229fb": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_texImage2D_a4011abffe0229fb"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32);
/******/ 					},
/******/ 					"__wbg_texImage2D_8861cbc8aa02485c": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_texImage2D_8861cbc8aa02485c"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix3fv_a0a261980af4587d": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_uniformMatrix3fv_a0a261980af4587d"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix4fv_088c96db8ee28c1d": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_uniformMatrix4fv_088c96db8ee28c1d"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_activeTexture_a51ec6273de88bc6": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_activeTexture_a51ec6273de88bc6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_attachShader_0dd248f6ab98fcf2": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_attachShader_0dd248f6ab98fcf2"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindBuffer_1ceb83e9674e812a": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_bindBuffer_1ceb83e9674e812a"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindTexture_6121e6db3f879582": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_bindTexture_6121e6db3f879582"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_blendEquation_da8519d0015abb5f": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_blendEquation_da8519d0015abb5f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_blendFunc_34a6bb31770822c5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_blendFunc_34a6bb31770822c5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_clear_f6b2dd48aeed2752": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_clear_f6b2dd48aeed2752"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_clearColor_89f7819aa9f80129": function(p0i32,p1f32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_clearColor_89f7819aa9f80129"](p0i32,p1f32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_compileShader_28bdbafe4445d24b": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_compileShader_28bdbafe4445d24b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createBuffer_acedc3831832a280": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_createBuffer_acedc3831832a280"](p0i32);
/******/ 					},
/******/ 					"__wbg_createProgram_7e2f44b7b74694d4": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_createProgram_7e2f44b7b74694d4"](p0i32);
/******/ 					},
/******/ 					"__wbg_createShader_64c474f1d1d0c1f8": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_createShader_64c474f1d1d0c1f8"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createTexture_0a156dab1efc3499": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_createTexture_0a156dab1efc3499"](p0i32);
/******/ 					},
/******/ 					"__wbg_drawArrays_604abf0ccb310fe7": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_drawArrays_604abf0ccb310fe7"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_enable_87f39f6396535e1f": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_enable_87f39f6396535e1f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_enableVertexAttribArray_f29c8dde9c8c5cf5": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_enableVertexAttribArray_f29c8dde9c8c5cf5"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_generateMipmap_7e65e4c36fe45473": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_generateMipmap_7e65e4c36fe45473"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getAttribLocation_ba61f837da80e249": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getAttribLocation_ba61f837da80e249"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_getProgramInfoLog_aacf06c959070653": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getProgramInfoLog_aacf06c959070653"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getProgramParameter_a89bf14502c109f7": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getProgramParameter_a89bf14502c109f7"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderInfoLog_1eb885f2468e2429": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getShaderInfoLog_1eb885f2468e2429"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderParameter_99510442d33c6589": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getShaderParameter_99510442d33c6589"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getUniformLocation_ca853de4f2f9270d": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getUniformLocation_ca853de4f2f9270d"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_linkProgram_46a36cb158f10676": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_linkProgram_46a36cb158f10676"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_shaderSource_700ae72fca39850d": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_shaderSource_700ae72fca39850d"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_texParameteri_e45f3977eb998137": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_texParameteri_e45f3977eb998137"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform1i_e76b668973ae0655": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_uniform1i_e76b668973ae0655"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_useProgram_d63a57db0571e803": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_useProgram_d63a57db0571e803"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribPointer_b4b829a4f5a3778e": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_vertexAttribPointer_b4b829a4f5a3778e"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_setonload_6fb79f39cb59de67": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setonload_6fb79f39cb59de67"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_keyCode_47f9e9228bc483bf": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_keyCode_47f9e9228bc483bf"](p0i32);
/******/ 					},
/******/ 					"__wbg_setsrc_ca894d724570195d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_setsrc_ca894d724570195d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_width_f3b349ce7a53895c": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_width_f3b349ce7a53895c"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_59f27baafc5c7528": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_height_59f27baafc5c7528"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_1f2ded01c889cd38": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_new_1f2ded01c889cd38"]();
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_7bd3ee7838f11fc3": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_7bd3ee7838f11fc3"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_3db9399e6dc524ff": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_getContext_3db9399e6dc524ff"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_get_5fa3f454aa041e6e": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_get_5fa3f454aa041e6e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_from_aee16bee83bf975b": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_from_aee16bee83bf975b"](p0i32);
/******/ 					},
/******/ 					"__wbg_length_d2491466819b6271": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_length_d2491466819b6271"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_7c6bd521992b4022": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_newnoargs_7c6bd521992b4022"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_951bd0c6d815d6f1": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_call_951bd0c6d815d6f1"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_globalThis_513fb247e8e4e6d2": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_globalThis_513fb247e8e4e6d2"]();
/******/ 					},
/******/ 					"__wbg_self_6baf3a3aa7b63415": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_self_6baf3a3aa7b63415"]();
/******/ 					},
/******/ 					"__wbg_window_63fc4027b66c265b": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_window_63fc4027b66c265b"]();
/******/ 					},
/******/ 					"__wbg_global_b87245cd886d7113": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_global_b87245cd886d7113"]();
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_9428545f18592c34": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_newwithbyteoffsetandlength_9428545f18592c34"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_buffer_3f12a1c608c6d04e": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbg_buffer_3f12a1c608c6d04e"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_string_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_boolean_get": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_boolean_get"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper963": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/index_bg.js"].exports["__wbindgen_closure_wrapper963"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"1":["./pkg/index_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/index_bg.wasm":"2c8fdb36f5738286e1d4"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./js/index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./assets/back2.jpg":
/*!**************************!*\
  !*** ./assets/back2.jpg ***!
  \**************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (__webpack_require__.p + \"59a9517be04e11756fda50f48d96459a.jpg\");\n\n//# sourceURL=webpack:///./assets/back2.jpg?");

/***/ }),

/***/ "./assets/beam.png":
/*!*************************!*\
  !*** ./assets/beam.png ***!
  \*************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAIAAACQkWg2AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAD7SURBVDhPlVG7SkNBFDzj3/sDgj8gYqvYCIJNQEidQrBIky4pbMQm6zx2r0u0MMPlMHvOzOzeXVzetjoHuH6wgQX+SMKD8FZAtZC7FxnQNBKpOrJy3OoidphQAMnwtGaxYUklFrcDJHWDwPrN4WxENKBGmtyNxVM533dd6NEJmzCaOHzIkE3V1AFF/gZlR/5OzFH9zh7pqWg2/B/4/OqO+STkutwsRjd3jd3+Z4dZsVwlV7qiPitstq0/gn6oWl5rgO1lFRlWGxmIKcV+QyN+UwoeX1s/nHVSjpc0FetHsB83z+IynLxo1LPTHFf3ClRGZpk7IiYVN0WqvgFF74H4n1dPzwAAAABJRU5ErkJggg==\");\n\n//# sourceURL=webpack:///./assets/beam.png?");

/***/ }),

/***/ "./assets/boss.png":
/*!*************************!*\
  !*** ./assets/boss.png ***!
  \*************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAMAAABEpIrGAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAMAUExURQAAAFpra1LnUs4ICP8QEL1C/4SlrYz3lP//lAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPrSdXgAAAEAdFJOU////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wBT9wclAAAACXBIWXMAAA7DAAAOwwHHb6hkAAAAGHRFWHRTb2Z0d2FyZQBwYWludC5uZXQgNC4xLjb9TgnoAAAA4ElEQVQ4T5WRCxKDIAxELQJy/wvT3WT5KW2nbxxNss8MrUf9gQnZyj1fBc5NePG2oQt9w0VUg4yAQtYGi68reAcYTELLu5GHYIYEGu7Y3ASoueduwPfNfsisaGUIGjyRoG6DC2r2fBZSwg2HPZRbP0gpnSeeYRImBSVSXGEVtBU1X6YAAwJGYBJO204CBc87IdirBv5SCDdjFnzD/XfCaPCjUVhWrLkLOibBTnV8ontuAJZ5/thAPHH0udUhjDHOeRNqLaVQQx5jwZy9Mwm8M/dqKwz+FzBR4Yy2CavRm1rffdKnmVtAdYcAAAAASUVORK5CYII=\");\n\n//# sourceURL=webpack:///./assets/boss.png?");

/***/ }),

/***/ "./assets/bullet.png":
/*!***************************!*\
  !*** ./assets/bullet.png ***!
  \***************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAgAAAAQCAMAAAAcVM5PAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAMAUExURQAAADEhIWMpIYQhIbUhEK0xIcYxEM5SEO+ECPetKf/WOf//WgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAqVCdMAAAEAdFJOU////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wBT9wclAAAACXBIWXMAAA7DAAAOwwHHb6hkAAAAGHRFWHRTb2Z0d2FyZQBwYWludC5uZXQgNC4xLjb9TgnoAAAAP0lEQVQYV23NMRLAIAwDQewkEMj/36ucNZRortjKbtoDFyvkl3RErKDCDAJ6HRgOdAduBx4HuO8XSgfCgZr0A0ESTo/6eu0uAAAAAElFTkSuQmCC\");\n\n//# sourceURL=webpack:///./assets/bullet.png?");

/***/ }),

/***/ "./assets/ebullet.png":
/*!****************************!*\
  !*** ./assets/ebullet.png ***!
  \****************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAbUExURQAAAEhIAP//AP9qBv9OAupSAPdKAP8aAAAAAJgTbxcAAAAJdFJOU///////////AFNPeBIAAAAJcEhZcwAADsIAAA7CARUoSoAAAABESURBVChTjc7JDQAgCARAUUD6r1hwP4qJui+YhKNYyjeQBxWAqgcygWpjbpAA70VVIADW3pVvkEaOpSH7WZf02JIXmA1w/QZ5HyJRRQAAAABJRU5ErkJggg==\");\n\n//# sourceURL=webpack:///./assets/ebullet.png?");

/***/ }),

/***/ "./assets/enemy.png":
/*!**************************!*\
  !*** ./assets/enemy.png ***!
  \**************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAMAUExURQAAAP8pKc5aWpyMhAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAG4nl3MAAAEAdFJOU////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wBT9wclAAAACXBIWXMAAA7DAAAOwwHHb6hkAAAAGHRFWHRTb2Z0d2FyZQBwYWludC5uZXQgNC4xLjb9TgnoAAAAT0lEQVQoU3XMQQ6AQAwCQKX/f3OlQDUxWQ4rTGKv/oVQqQxrAMwHVbMBFkFnUzgGMhkDcHvysyCZN6AuXXhzBN9Y8JGcEOxfqgaJW2DT/QA/XXyJ2NguQAAAAABJRU5ErkJggg==\");\n\n//# sourceURL=webpack:///./assets/enemy.png?");

/***/ }),

/***/ "./assets/explode.png":
/*!****************************!*\
  !*** ./assets/explode.png ***!
  \****************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAAAQCAMAAADphoe6AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAMAUExURQAAAHtzAK0QAPcYAPdjGP+UEP+tGP/3EPfnc////wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAKZ+U2gAAAEAdFJOU////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wBT9wclAAAACXBIWXMAAA7DAAAOwwHHb6hkAAAAGHRFWHRTb2Z0d2FyZQBwYWludC5uZXQgNC4xLjb9TgnoAAABjklEQVRIS61Ti3LDIAxbbfPI/39wJhsDTlh2XDa1d0TCRcJOv85/APvaQb7u4FUA8XVgI8Fd6vxVgLvfKpxE5sAtKxmH0tTJdwNkXzsWQ5ynx43WKIfidfBi7uZA5E8BZrVB6OrJxLEA50GYNc57CXYkq6VT7KOi8YcALHJxkJzTjEAiOVFypn4qpPEL9ctJZgCWJHpiT6DxlOHrAT7l0x4aOJcsIQCB5/nqSS7YnwXww36SHhEcjojsFegHEbTOYa7e1rEW4FOPGhKQ1FrzbAFJKRV3dnqqf0mDE2fceArKmRDBh9CuC+4B3VpH5h2A/xFbAP8jtpxLLSX1DuDuBVeWxgXDxEB1BCboy6d+mIrNQL92X+emmGSf3oHrCNhGEDrwE4evUxMkBQH3tbeoC3hAxvgDSG374SXEwGZ1G+rKw3mww8SHsHDtEbgL1iUL8fw3DKc3zIEYwm0U1nZ/VvzK9cH8lTwGWFB97Zh/Ckc03Md+gDuKrwO3Hm0Geh9gacnSk60Efwiw4EWA8/wGfbgayBUmONgAAAAASUVORK5CYII=\");\n\n//# sourceURL=webpack:///./assets/explode.png?");

/***/ }),

/***/ "./assets/explode2.png":
/*!*****************************!*\
  !*** ./assets/explode2.png ***!
  \*****************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAAgCAMAAADKd1bWAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAeUExURQAAAP/3Wv9rIa0ICHsAAP/OMf+lMf9CAP8AAAAAAD/BtUQAAAAKdFJOU////////////wCyzCzPAAAACXBIWXMAAA7CAAAOwgEVKEqAAAADjklEQVRoQ9VZi3brIAy7ITRJ//+Hr2RcCokdRkJ3OrUbD7EUyTbpyf49P4FJWwchaMfBPGvHQdR2BD5iwENbB7f1f7sBy6IdB+uqHQdNgV9vQCPC210DRuKSATdL/M8b0Crxhr5527T3DbhiwM0Sb51xzRIfmiAXDAjnBmyNAMfYMuBXccUAbROmJdQh32U42G0uyzrWJT7Pu4kdXiSaqK+h6DfgmOErgp6jGuc6xQNYitThAZUB6GFUrX0PhOPYvdYV/NiAfPJbZzgkvlRbAW0FWRuRT51pXAFzmTcXXMXPM0Du7hEJbpZ4ciAyF2Z/f0wPl6e8xFt2iTORXDJhFGoDoM/fPmp53laUtLkCHMxZUfK2QvIADermqRgG48UDZqwDpQGQF0J4l/MeiQ+rtYDBUxoeHveXgruFJNDl1YDjAs6QhwvU/xkD4rosjyV4d7E4rwH8ZC7AnqE/0VaIwTNBmEAmjwWMMUiLx1h55gDlG1e4hqoEIGB6LKtzcdEPPlg1kPTDANwVrTICTwOYII5BmMYK5Bf5wwLyLFC8Uwk4m+xGlQHUB4F2DSAB1gW0bwAdWKQGjldgAGUFTgmTR4LTISwhv/sEapb44xTEXcgokcsoDdgQYej3MoABVIE6UyAleCCNU/J4BUZPEsDjmQEbkky+VNi8HIQsEvl7Z5e9qEogpaidAKJhAr+YCxhgOkAB5iHHADO8Hs9DguJZIRjptEDkCo/PeFI+34m7jcoA7vLkwvh44GFsH8D+JINFgbEi83LK62QBSGIK8GX8vVQ90p+LaMYw/bUBDVACE1CHO0gOiHzbRbpLiaRNnqcAeX7b0TkFhskB8swFnR+BHgPkTuzpl+2LQyhX60aZePzYCjjLg0LoAw/1BHh05K3EbfQZAHjfEkR5ipDzyEd4Tz8gQRbW8ifz0r7m7qPbAO9xB04/bI8COTg8FJPNJt7dt6gHdFhDxJe/BqHbAOeBnz4IhUBp90+Nki8lagk5qid4rximHug2YPc8ZAccAtqrgdrWngIxLGZYF9rNsIKcpkbqv2DA+SNRnILaK7HlbxdZKQ601CG0ckqwXLRbYbQH/QacPxPHnV57Jd6Thaz3JPQfxfKmq90XatljDoJ+AxowDSjKppCaZ8tkKFTlbHKUDnFguAHN/4oYJVLOWaJcpQMc+H0DjAwpPekyYACGG9D6v9HTKpESlgHajsfz+R+1OgywGyO7JQAAAABJRU5ErkJggg==\");\n\n//# sourceURL=webpack:///./assets/explode2.png?");

/***/ }),

/***/ "./assets/missile.png":
/*!****************************!*\
  !*** ./assets/missile.png ***!
  \****************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAMAUExURQAAAGMQEEJaY2OEjHOUpYwYGMYQEPd7If/GKf/vKf///wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAL9NxfsAAAEAdFJOU////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wBT9wclAAAACXBIWXMAAA7DAAAOwwHHb6hkAAAAGHRFWHRTb2Z0d2FyZQBwYWludC5uZXQgNC4xLjb9TgnoAAAAVUlEQVQoU53NSRLAIAhE0WgLDve/LxEbrMo2bPy8BT7GASICGlBZhOZA+QsgnI8ckOCyAQ51z4684evnqNW1uCf0OTsrQMdQVoCoCivAJPYLpZzH7AV/C6qL4LtQaAAAAABJRU5ErkJggg==\");\n\n//# sourceURL=webpack:///./assets/missile.png?");

/***/ }),

/***/ "./assets/player.png":
/*!***************************!*\
  !*** ./assets/player.png ***!
  \***************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAMAAABEpIrGAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAMAUExURQAAABcXF25ubhBz3jGU///OQv/ea4SEhJycnLW1tc7OzgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALW9FvoAAAEAdFJOU////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////wBT9wclAAAACXBIWXMAAA7DAAAOwwHHb6hkAAAAGHRFWHRTb2Z0d2FyZQBwYWludC5uZXQgNC4xLjb9TgnoAAAAkklEQVQ4T83SwQ6EMAgE0EW3Av3//+1OBcyWmuhl487BTODZQ9NXu8g/AiIvkQRINYnbwI7GXhdr4RyQzUnfqxdzCVDtYG+ngGpVkaWLpwBpAEafAa7gACIzUJVvIDIA7DEaAZsI0Ec4ZQf8U8CWM0AIVqXg0/sI4jlw2bZim3gQDjwQvj8yAvyXX20Cc54HrX0A4xi0ZuKgtMUAAAAASUVORK5CYII=\");\n\n//# sourceURL=webpack:///./assets/player.png?");

/***/ }),

/***/ "./assets/trail.png":
/*!**************************!*\
  !*** ./assets/trail.png ***!
  \**************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAYAAADED76LAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAB2SURBVChTfY/ZEYMwEENXuCkKSw+pjl8qgSGTCRZPJt+sR3toT8t2PYn88YuSVqpTYVwT+gQNTPLhBSe5kEmWR2W6DXZ/w4XU2GaeSgkszPZ8RArWv99Bhgdp6qhM6G/MPTki7rg9NDsomDGNML/QSHLC0K7fBbb9OnmANtlXAAAAAElFTkSuQmCC\");\n\n//# sourceURL=webpack:///./assets/trail.png?");

/***/ }),

/***/ "./js/index.js":
/*!*********************!*\
  !*** ./js/index.js ***!
  \*********************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _assets_enemy_png__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../assets/enemy.png */ \"./assets/enemy.png\");\n/* harmony import */ var _assets_player_png__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../assets/player.png */ \"./assets/player.png\");\n/* harmony import */ var _assets_bullet_png__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../assets/bullet.png */ \"./assets/bullet.png\");\n/* harmony import */ var _assets_ebullet_png__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../assets/ebullet.png */ \"./assets/ebullet.png\");\n/* harmony import */ var _assets_boss_png__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ../assets/boss.png */ \"./assets/boss.png\");\n/* harmony import */ var _assets_missile_png__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ../assets/missile.png */ \"./assets/missile.png\");\n/* harmony import */ var _assets_explode_png__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ../assets/explode.png */ \"./assets/explode.png\");\n/* harmony import */ var _assets_explode2_png__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ../assets/explode2.png */ \"./assets/explode2.png\");\n/* harmony import */ var _assets_trail_png__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ../assets/trail.png */ \"./assets/trail.png\");\n/* harmony import */ var _assets_beam_png__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! ../assets/beam.png */ \"./assets/beam.png\");\n/* harmony import */ var _assets_back2_jpg__WEBPACK_IMPORTED_MODULE_10__ = __webpack_require__(/*! ../assets/back2.jpg */ \"./assets/back2.jpg\");\n\n\n\n\n\n\n\n\n\n\n\n\n// Note that a dynamic `import` statement here is required due to\n// webpack/webpack#6615, but in theory `import { greet } from './pkg';`\n// will work here one day as well!\nconst rust = Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.bind(null, /*! ../pkg */ \"./pkg/index.js\"));\n\nrust\n  .catch(console.error)\n  .then((module) => {\n    let image_assets = [\n      [\"enemy\", _assets_enemy_png__WEBPACK_IMPORTED_MODULE_0__[\"default\"]],\n      [\"player\", _assets_player_png__WEBPACK_IMPORTED_MODULE_1__[\"default\"]],\n      [\"bullet\", _assets_bullet_png__WEBPACK_IMPORTED_MODULE_2__[\"default\"]],\n      [\"ebullet\", _assets_ebullet_png__WEBPACK_IMPORTED_MODULE_3__[\"default\"]],\n      [\"boss\", _assets_boss_png__WEBPACK_IMPORTED_MODULE_4__[\"default\"]],\n      [\"missile\", _assets_missile_png__WEBPACK_IMPORTED_MODULE_5__[\"default\"]],\n      [\"explode\", _assets_explode_png__WEBPACK_IMPORTED_MODULE_6__[\"default\"]],\n      [\"explode2\", _assets_explode2_png__WEBPACK_IMPORTED_MODULE_7__[\"default\"]],\n      [\"trail\", _assets_trail_png__WEBPACK_IMPORTED_MODULE_8__[\"default\"]],\n      [\"beam\", _assets_beam_png__WEBPACK_IMPORTED_MODULE_9__[\"default\"]],\n      [\"back\", _assets_back2_jpg__WEBPACK_IMPORTED_MODULE_10__[\"default\"]],\n    ];\n    let state = new module.ShooterState(image_assets);\n\n    state.start();\n\n    document.addEventListener(\"keydown\", (event) => state.key_down(event));\n    document.addEventListener(\"keyup\", (event) => state.key_up(event));\n\n    function render() {\n      state.render();\n      requestAnimationFrame(render);\n    }\n    requestAnimationFrame(render);\n  });\n\n\n//# sourceURL=webpack:///./js/index.js?");

/***/ })

/******/ });