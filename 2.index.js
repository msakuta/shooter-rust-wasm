(window.webpackJsonp=window.webpackJsonp||[]).push([[2],{1:function(n,t,r){"use strict";(function(n,e,u,o){let _;function c(n){_=n}r.d(t,"V",(function(){return c})),r.d(t,"a",(function(){return E})),r.d(t,"b",(function(){return F})),r.d(t,"c",(function(){return M})),r.d(t,"d",(function(){return j})),r.d(t,"e",(function(){return D})),r.d(t,"f",(function(){return O})),r.d(t,"g",(function(){return B})),r.d(t,"h",(function(){return q})),r.d(t,"i",(function(){return R})),r.d(t,"j",(function(){return W})),r.d(t,"k",(function(){return z})),r.d(t,"l",(function(){return H})),r.d(t,"m",(function(){return G})),r.d(t,"n",(function(){return N})),r.d(t,"o",(function(){return V})),r.d(t,"p",(function(){return $})),r.d(t,"q",(function(){return J})),r.d(t,"r",(function(){return U})),r.d(t,"s",(function(){return K})),r.d(t,"t",(function(){return Q})),r.d(t,"u",(function(){return X})),r.d(t,"v",(function(){return Y})),r.d(t,"w",(function(){return Z})),r.d(t,"x",(function(){return nn})),r.d(t,"y",(function(){return tn})),r.d(t,"z",(function(){return rn})),r.d(t,"A",(function(){return en})),r.d(t,"B",(function(){return un})),r.d(t,"C",(function(){return on})),r.d(t,"D",(function(){return _n})),r.d(t,"E",(function(){return cn})),r.d(t,"F",(function(){return fn})),r.d(t,"G",(function(){return dn})),r.d(t,"H",(function(){return an})),r.d(t,"I",(function(){return bn})),r.d(t,"J",(function(){return gn})),r.d(t,"K",(function(){return sn})),r.d(t,"L",(function(){return ln})),r.d(t,"M",(function(){return wn})),r.d(t,"N",(function(){return hn})),r.d(t,"O",(function(){return mn})),r.d(t,"P",(function(){return yn})),r.d(t,"Q",(function(){return pn})),r.d(t,"R",(function(){return In})),r.d(t,"S",(function(){return vn})),r.d(t,"T",(function(){return xn})),r.d(t,"U",(function(){return kn})),r.d(t,"W",(function(){return An})),r.d(t,"X",(function(){return Sn})),r.d(t,"Y",(function(){return Ln})),r.d(t,"Z",(function(){return Pn})),r.d(t,"ab",(function(){return Tn})),r.d(t,"bb",(function(){return Cn})),r.d(t,"cb",(function(){return En})),r.d(t,"db",(function(){return Fn})),r.d(t,"eb",(function(){return Mn})),r.d(t,"fb",(function(){return jn})),r.d(t,"gb",(function(){return Dn})),r.d(t,"hb",(function(){return On})),r.d(t,"ib",(function(){return Bn})),r.d(t,"jb",(function(){return qn})),r.d(t,"kb",(function(){return Rn})),r.d(t,"lb",(function(){return Wn})),r.d(t,"mb",(function(){return zn})),r.d(t,"nb",(function(){return Hn})),r.d(t,"ob",(function(){return Gn})),r.d(t,"pb",(function(){return Nn})),r.d(t,"qb",(function(){return Vn})),r.d(t,"rb",(function(){return $n})),r.d(t,"sb",(function(){return Jn})),r.d(t,"tb",(function(){return Un})),r.d(t,"ub",(function(){return Kn})),r.d(t,"vb",(function(){return Qn})),r.d(t,"wb",(function(){return Xn})),r.d(t,"xb",(function(){return Yn})),r.d(t,"yb",(function(){return Zn})),r.d(t,"zb",(function(){return nt}));const i=new Array(128).fill(void 0);function f(n){return i[n]}i.push(void 0,null,!0,!1);let d=i.length;function a(n){d===i.length&&i.push(i.length+1);const t=d;return d=i[t],i[t]=n,t}function b(n,t){try{return n.apply(this,t)}catch(n){_.__wbindgen_exn_store(a(n))}}function g(n){return null==n}let s=new(void 0===n?(0,e.require)("util").TextDecoder:n)("utf-8",{ignoreBOM:!0,fatal:!0});s.decode();let l=null;function w(){return null!==l&&0!==l.byteLength||(l=new Uint8Array(_.memory.buffer)),l}function h(n,t){return n>>>=0,s.decode(w().subarray(n,n+t))}let m=0;let y=new(void 0===u?(0,e.require)("util").TextEncoder:u)("utf-8");const p="function"==typeof y.encodeInto?function(n,t){return y.encodeInto(n,t)}:function(n,t){const r=y.encode(n);return t.set(r),{read:n.length,written:r.length}};function I(n,t,r){if(void 0===r){const r=y.encode(n),e=t(r.length,1)>>>0;return w().subarray(e,e+r.length).set(r),m=r.length,e}let e=n.length,u=t(e,1)>>>0;const o=w();let _=0;for(;_<e;_++){const t=n.charCodeAt(_);if(t>127)break;o[u+_]=t}if(_!==e){0!==_&&(n=n.slice(_)),u=r(u,e,e=_+3*n.length,1)>>>0;const t=w().subarray(u+_,u+e);_+=p(n,t).written,u=r(u,e,_,1)>>>0}return m=_,u}let v=null;function x(){return(null===v||!0===v.buffer.detached||void 0===v.buffer.detached&&v.buffer!==_.memory.buffer)&&(v=new DataView(_.memory.buffer)),v}function k(n,t){return n>>>=0,w().subarray(n/1,n/1+t)}let A=null;function S(n,t){return n>>>=0,(null!==A&&0!==A.byteLength||(A=new Float32Array(_.memory.buffer)),A).subarray(n/4,n/4+t)}const L="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(n=>{_.__wbindgen_export_3.get(n.dtor)(n.a,n.b)});function P(n){const t=f(n);return function(n){n<132||(i[n]=d,d=n)}(n),t}function T(n,t){try{const e=_.__wbindgen_add_to_stack_pointer(-16);_._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h67c20f0f023fa6d5(e,n,t);var r=x().getInt32(e+0,!0);if(x().getInt32(e+4,!0))throw P(r)}finally{_.__wbindgen_add_to_stack_pointer(16)}}const C="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(n=>_.__wbg_shooterstate_free(n>>>0,1));class E{__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,C.unregister(this),n}free(){const n=this.__destroy_into_raw();_.__wbg_shooterstate_free(n,0)}constructor(n){try{const e=_.__wbindgen_add_to_stack_pointer(-16);_.shooterstate_new(e,a(n));var t=x().getInt32(e+0,!0),r=x().getInt32(e+4,!0);if(x().getInt32(e+8,!0))throw P(r);return this.__wbg_ptr=t>>>0,C.register(this,this.__wbg_ptr,this),this}finally{_.__wbindgen_add_to_stack_pointer(16)}}key_down(n){try{const e=_.__wbindgen_add_to_stack_pointer(-16);_.shooterstate_key_down(e,this.__wbg_ptr,a(n));var t=x().getInt32(e+0,!0),r=x().getInt32(e+4,!0);if(x().getInt32(e+8,!0))throw P(r);return P(t)}finally{_.__wbindgen_add_to_stack_pointer(16)}}key_up(n){_.shooterstate_key_up(this.__wbg_ptr,a(n))}restart(){try{const t=_.__wbindgen_add_to_stack_pointer(-16);_.shooterstate_restart(t,this.__wbg_ptr);var n=x().getInt32(t+0,!0);if(x().getInt32(t+4,!0))throw P(n)}finally{_.__wbindgen_add_to_stack_pointer(16)}}start(){try{const t=_.__wbindgen_add_to_stack_pointer(-16);_.shooterstate_start(t,this.__wbg_ptr);var n=x().getInt32(t+0,!0);if(x().getInt32(t+4,!0))throw P(n)}finally{_.__wbindgen_add_to_stack_pointer(16)}}render(){try{const t=_.__wbindgen_add_to_stack_pointer(-16);_.shooterstate_render(t,this.__wbg_ptr);var n=x().getInt32(t+0,!0);if(x().getInt32(t+4,!0))throw P(n)}finally{_.__wbindgen_add_to_stack_pointer(16)}}}function F(n,t){f(n).activeTexture(t>>>0)}function M(){return b((function(n,t){return a(f(n).appendChild(f(t)))}),arguments)}function j(n,t,r){f(n).attachShader(f(t),f(r))}function D(n,t,r){f(n).bindBuffer(t>>>0,f(r))}function O(n,t,r){f(n).bindTexture(t>>>0,f(r))}function B(n,t){f(n).blendEquation(t>>>0)}function q(n,t,r){f(n).blendFunc(t>>>0,r>>>0)}function R(n,t,r,e){f(n).bufferData(t>>>0,f(r),e>>>0)}function W(n){return a(f(n).buffer)}function z(){return b((function(n,t){return a(f(n).call(f(t)))}),arguments)}function H(n,t,r,e,u){f(n).clearColor(t,r,e,u)}function G(n,t){f(n).clear(t>>>0)}function N(n,t){f(n).compileShader(f(t))}function V(n){const t=f(n).createBuffer();return g(t)?0:a(t)}function $(){return b((function(n,t,r){return a(f(n).createElement(h(t,r)))}),arguments)}function J(n){const t=f(n).createProgram();return g(t)?0:a(t)}function U(n,t){const r=f(n).createShader(t>>>0);return g(r)?0:a(r)}function K(n){const t=f(n).createTexture();return g(t)?0:a(t)}function Q(n){const t=f(n).document;return g(t)?0:a(t)}function X(n,t,r,e){f(n).drawArrays(t>>>0,r,e)}function Y(n,t){f(n).enableVertexAttribArray(t>>>0)}function Z(n,t){f(n).enable(t>>>0)}function nn(n){return a(Array.from(f(n)))}function tn(n,t){f(n).generateMipmap(t>>>0)}function rn(n,t,r,e){return f(n).getAttribLocation(f(t),h(r,e))}function en(){return b((function(n,t,r){const e=f(n).getContext(h(t,r));return g(e)?0:a(e)}),arguments)}function un(n,t,r){const e=f(n).getElementById(h(t,r));return g(e)?0:a(e)}function on(n,t,r){const e=f(t).getProgramInfoLog(f(r));var u=g(e)?0:I(e,_.__wbindgen_malloc,_.__wbindgen_realloc),o=m;x().setInt32(n+4,o,!0),x().setInt32(n+0,u,!0)}function _n(n,t,r){return a(f(n).getProgramParameter(f(t),r>>>0))}function cn(n,t,r){const e=f(t).getShaderInfoLog(f(r));var u=g(e)?0:I(e,_.__wbindgen_malloc,_.__wbindgen_realloc),o=m;x().setInt32(n+4,o,!0),x().setInt32(n+0,u,!0)}function fn(n,t,r){return a(f(n).getShaderParameter(f(t),r>>>0))}function dn(n,t,r,e){const u=f(n).getUniformLocation(f(t),h(r,e));return g(u)?0:a(u)}function an(n,t){return a(f(n)[t>>>0])}function bn(n){return f(n).height}function gn(n){let t;try{t=f(n)instanceof HTMLCanvasElement}catch(n){t=!1}return t}function sn(n){let t;try{t=f(n)instanceof WebGLRenderingContext}catch(n){t=!1}return t}function ln(n){let t;try{t=f(n)instanceof Window}catch(n){t=!1}return t}function wn(n){return f(n).keyCode}function hn(n){return f(n).length}function mn(n,t){f(n).linkProgram(f(t))}function yn(n,t){console.log(h(n,t))}function pn(n,t){console.log(h(n,t))}function In(){return b((function(){return a(new Image)}),arguments)}function vn(n,t){return a(new Function(h(n,t)))}function xn(n,t,r){return a(new Float32Array(f(n),t>>>0,r>>>0))}function kn(){return b((function(n,t,r,e,u){f(n).setAttribute(h(t,r),h(e,u))}),arguments)}function An(n,t,r){f(n).className=h(t,r)}function Sn(n,t,r){f(n).innerHTML=h(t,r)}function Ln(n,t){f(n).onload=f(t)}function Pn(n,t,r){f(n).src=h(t,r)}function Tn(n,t,r,e){f(n).shaderSource(f(t),h(r,e))}function Cn(){const n=void 0===o?null:o;return g(n)?0:a(n)}function En(){const n="undefined"==typeof globalThis?null:globalThis;return g(n)?0:a(n)}function Fn(){const n="undefined"==typeof self?null:self;return g(n)?0:a(n)}function Mn(){const n="undefined"==typeof window?null:window;return g(n)?0:a(n)}function jn(){return b((function(n,t,r,e,u,o,_){f(n).texImage2D(t>>>0,r,e,u>>>0,o>>>0,f(_))}),arguments)}function Dn(){return b((function(n,t,r,e,u,o,_,c,i,d,a){f(n).texImage2D(t>>>0,r,e,u,o,_,c>>>0,i>>>0,0===d?void 0:k(d,a))}),arguments)}function On(n,t,r,e){f(n).texParameteri(t>>>0,r>>>0,e)}function Bn(n,t,r){f(n).uniform1f(f(t),r)}function qn(n,t,r){f(n).uniform1i(f(t),r)}function Rn(n,t,r,e,u){f(n).uniformMatrix3fv(f(t),0!==r,S(e,u))}function Wn(n,t,r,e,u){f(n).uniformMatrix4fv(f(t),0!==r,S(e,u))}function zn(n,t){f(n).useProgram(f(t))}function Hn(n,t,r,e,u,o,_){f(n).vertexAttribPointer(t>>>0,r,e>>>0,0!==u,o,_)}function Gn(n){return f(n).width}function Nn(n){const t=f(n);return"boolean"==typeof t?t?1:0:2}function Vn(n,t,r){return a(function(n,t,r,e){const u={a:n,b:t,cnt:1,dtor:r},o=(...n)=>{u.cnt++;const t=u.a;u.a=0;try{return e(t,u.b,...n)}finally{0==--u.cnt?(_.__wbindgen_export_3.get(u.dtor)(t,u.b),L.unregister(u)):u.a=t}};return o.original=u,L.register(o,u,u),o}(n,t,42,T))}function $n(n,t){const r=I(function n(t){const r=typeof t;if("number"==r||"boolean"==r||null==t)return""+t;if("string"==r)return`"${t}"`;if("symbol"==r){const n=t.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==r){const n=t.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(t)){const r=t.length;let e="[";r>0&&(e+=n(t[0]));for(let u=1;u<r;u++)e+=", "+n(t[u]);return e+="]",e}const e=/\[object ([^\]]+)\]/.exec(toString.call(t));let u;if(!(e&&e.length>1))return toString.call(t);if(u=e[1],"Object"==u)try{return"Object("+JSON.stringify(t)+")"}catch(n){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:u}(f(t)),_.__wbindgen_malloc,_.__wbindgen_realloc),e=m;x().setInt32(n+4,e,!0),x().setInt32(n+0,r,!0)}function Jn(n){return void 0===f(n)}function Un(n,t){return f(n)===f(t)}function Kn(){return a(_.memory)}function Qn(n){return a(f(n))}function Xn(n){P(n)}function Yn(n,t){const r=f(t),e="string"==typeof r?r:void 0;var u=g(e)?0:I(e,_.__wbindgen_malloc,_.__wbindgen_realloc),o=m;x().setInt32(n+4,o,!0),x().setInt32(n+0,u,!0)}function Zn(n,t){return a(h(n,t))}function nt(n,t){throw new Error(h(n,t))}}).call(this,r(2).TextDecoder,r(5)(n),r(2).TextEncoder,r(6))},7:function(n,t,r){"use strict";var e=r.w[n.i];for(var u in r.r(t),e)"__webpack_init__"!=u&&(t[u]=e[u]);r(1);e.__webpack_init__()},8:function(n,t,r){"use strict";r.r(t);var e=r(7),u=r(1);r.d(t,"__wbg_set_wasm",(function(){return u.V})),r.d(t,"ShooterState",(function(){return u.a})),r.d(t,"__wbg_activeTexture_446c979476d36a40",(function(){return u.b})),r.d(t,"__wbg_appendChild_d22bc7af6b96b3f1",(function(){return u.c})),r.d(t,"__wbg_attachShader_4dc5977795b5d865",(function(){return u.d})),r.d(t,"__wbg_bindBuffer_ff7c55f1062014bc",(function(){return u.e})),r.d(t,"__wbg_bindTexture_8b97cf7511a725d0",(function(){return u.f})),r.d(t,"__wbg_blendEquation_9f73e32730d0c986",(function(){return u.g})),r.d(t,"__wbg_blendFunc_57545f7f7240fd88",(function(){return u.h})),r.d(t,"__wbg_bufferData_7e2b6059c35c9291",(function(){return u.i})),r.d(t,"__wbg_buffer_61b7ce01341d7f88",(function(){return u.j})),r.d(t,"__wbg_call_b0d8e36992d9900d",(function(){return u.k})),r.d(t,"__wbg_clearColor_d58166c97d5eef07",(function(){return u.l})),r.d(t,"__wbg_clear_16ffdcc1a1d6f0c9",(function(){return u.m})),r.d(t,"__wbg_compileShader_afcc43901f14a922",(function(){return u.n})),r.d(t,"__wbg_createBuffer_567b536a03db30d2",(function(){return u.o})),r.d(t,"__wbg_createElement_89923fcb809656b7",(function(){return u.p})),r.d(t,"__wbg_createProgram_e2141127012594b0",(function(){return u.q})),r.d(t,"__wbg_createShader_442f69b8f536a786",(function(){return u.r})),r.d(t,"__wbg_createTexture_677a150f3f985ce0",(function(){return u.s})),r.d(t,"__wbg_document_f11bc4f7c03e1745",(function(){return u.t})),r.d(t,"__wbg_drawArrays_01e26acf05821932",(function(){return u.u})),r.d(t,"__wbg_enableVertexAttribArray_60827f2a43782639",(function(){return u.v})),r.d(t,"__wbg_enable_2bacfac56e802b11",(function(){return u.w})),r.d(t,"__wbg_from_d68eaa96dba25449",(function(){return u.x})),r.d(t,"__wbg_generateMipmap_82e271fcb6f70fdc",(function(){return u.y})),r.d(t,"__wbg_getAttribLocation_e104a96119fd0bbd",(function(){return u.z})),r.d(t,"__wbg_getContext_5eaf5645cd6acb46",(function(){return u.A})),r.d(t,"__wbg_getElementById_dcc9f1f3cfdca0bc",(function(){return u.B})),r.d(t,"__wbg_getProgramInfoLog_70d114345e15d2c1",(function(){return u.C})),r.d(t,"__wbg_getProgramParameter_d328869400b82698",(function(){return u.D})),r.d(t,"__wbg_getShaderInfoLog_23dd787b504d5f4e",(function(){return u.E})),r.d(t,"__wbg_getShaderParameter_e9098a633e6cf618",(function(){return u.F})),r.d(t,"__wbg_getUniformLocation_95f3933486db473c",(function(){return u.G})),r.d(t,"__wbg_get_9aa3dff3f0266054",(function(){return u.H})),r.d(t,"__wbg_height_a9968d3f0e288300",(function(){return u.I})),r.d(t,"__wbg_instanceof_HtmlCanvasElement_f764441ef5ddb63f",(function(){return u.J})),r.d(t,"__wbg_instanceof_WebGlRenderingContext_934db43ae44dbdac",(function(){return u.K})),r.d(t,"__wbg_instanceof_Window_d2514c6a7ee7ba60",(function(){return u.L})),r.d(t,"__wbg_keyCode_e673401ed53dfc2c",(function(){return u.M})),r.d(t,"__wbg_length_d65cf0786bfc5739",(function(){return u.N})),r.d(t,"__wbg_linkProgram_9b1029885a37b70d",(function(){return u.O})),r.d(t,"__wbg_log_51c176417106cb7b",(function(){return u.P})),r.d(t,"__wbg_log_e4402b6cc8d65d69",(function(){return u.Q})),r.d(t,"__wbg_new_c60de43ba24f2df6",(function(){return u.R})),r.d(t,"__wbg_newnoargs_fd9e4bf8be2bc16d",(function(){return u.S})),r.d(t,"__wbg_newwithbyteoffsetandlength_f113a96374814bb2",(function(){return u.T})),r.d(t,"__wbg_setAttribute_148e0e65e20e5f27",(function(){return u.U})),r.d(t,"__wbg_setclassName_6231eaf252b16a5b",(function(){return u.W})),r.d(t,"__wbg_setinnerHTML_2d75307ba8832258",(function(){return u.X})),r.d(t,"__wbg_setonload_482d0d1cba560c01",(function(){return u.Y})),r.d(t,"__wbg_setsrc_99e2795c356bc837",(function(){return u.Z})),r.d(t,"__wbg_shaderSource_6a657afd48edb05a",(function(){return u.ab})),r.d(t,"__wbg_static_accessor_GLOBAL_0be7472e492ad3e3",(function(){return u.bb})),r.d(t,"__wbg_static_accessor_GLOBAL_THIS_1a6eb482d12c9bfb",(function(){return u.cb})),r.d(t,"__wbg_static_accessor_SELF_1dc398a895c82351",(function(){return u.db})),r.d(t,"__wbg_static_accessor_WINDOW_ae1c80c7eea8d64a",(function(){return u.eb})),r.d(t,"__wbg_texImage2D_1139bbd3ce8e53f7",(function(){return u.fb})),r.d(t,"__wbg_texImage2D_ff073793ced7c108",(function(){return u.gb})),r.d(t,"__wbg_texParameteri_d550886a76f21258",(function(){return u.hb})),r.d(t,"__wbg_uniform1f_a8765c5b2bedff99",(function(){return u.ib})),r.d(t,"__wbg_uniform1i_fd66f39a37e6a753",(function(){return u.jb})),r.d(t,"__wbg_uniformMatrix3fv_072dfda2d6a0e388",(function(){return u.kb})),r.d(t,"__wbg_uniformMatrix4fv_b684a40949b2ff0b",(function(){return u.lb})),r.d(t,"__wbg_useProgram_88e7787408765ccf",(function(){return u.mb})),r.d(t,"__wbg_vertexAttribPointer_c6b1ccfa43bbca96",(function(){return u.nb})),r.d(t,"__wbg_width_ed3fd44e46b8c2c9",(function(){return u.ob})),r.d(t,"__wbindgen_boolean_get",(function(){return u.pb})),r.d(t,"__wbindgen_closure_wrapper209",(function(){return u.qb})),r.d(t,"__wbindgen_debug_string",(function(){return u.rb})),r.d(t,"__wbindgen_is_undefined",(function(){return u.sb})),r.d(t,"__wbindgen_jsval_eq",(function(){return u.tb})),r.d(t,"__wbindgen_memory",(function(){return u.ub})),r.d(t,"__wbindgen_object_clone_ref",(function(){return u.vb})),r.d(t,"__wbindgen_object_drop_ref",(function(){return u.wb})),r.d(t,"__wbindgen_string_get",(function(){return u.xb})),r.d(t,"__wbindgen_string_new",(function(){return u.yb})),r.d(t,"__wbindgen_throw",(function(){return u.zb})),Object(u.V)(e)}}]);
//# sourceMappingURL=2.index.js.map