// Interactive viewer for the SimForge asset explorer (templates/shortcodes/asset_explorer.html).
// It renders the glTF 2.0 binary files listed by the page with WebGL 2 and
// supports only what those files use: triangle meshes with float positions,
// normals and texture coordinates, and one PBR material with base colour,
// normal and metallic-roughness textures stored as PNG, JPEG or WebP
// (EXT_texture_webp). Nothing is fetched until the visitor starts the viewer.
(function () {
    "use strict";

    var FOV = 35 * Math.PI / 180;
    var MIN_PITCH = -1.35;
    var MAX_PITCH = 1.35;

    var VERTEX_SHADER = [
        "#version 300 es",
        "in vec3 inPosition;",
        "in vec3 inNormal;",
        "in vec2 inUv;",
        "uniform mat4 uModel;",
        "uniform mat4 uViewProjection;",
        "uniform mat3 uNormalMatrix;",
        "out vec3 vPosition;",
        "out vec3 vNormal;",
        "out vec2 vUv;",
        "void main() {",
        "  vec4 world = uModel * vec4(inPosition, 1.0);",
        "  vPosition = world.xyz;",
        "  vNormal = uNormalMatrix * inNormal;",
        "  vUv = inUv;",
        "  gl_Position = uViewProjection * world;",
        "}"
    ].join("\n");

    // Lambert diffuse plus GGX specular from a key and a rim light, with a
    // sky/ground gradient standing in for an environment map so that metals
    // keep visible reflections. The normal map uses a screen-space tangent
    // frame because the exported meshes carry no tangents.
    var FRAGMENT_SHADER = [
        "#version 300 es",
        "precision highp float;",
        "in vec3 vPosition;",
        "in vec3 vNormal;",
        "in vec2 vUv;",
        "uniform sampler2D uBase;",
        "uniform sampler2D uNormal;",
        "uniform sampler2D uMetalRough;",
        "uniform vec4 uBaseFactor;",
        "uniform float uMetallic;",
        "uniform float uRoughness;",
        "uniform float uNormalScale;",
        "uniform vec3 uCamera;",
        "uniform vec3 uKey;",
        "uniform vec3 uRim;",
        "out vec4 outColor;",
        "const float PI = 3.14159265;",
        "vec3 sky(vec3 d, float rough) {",
        "  vec3 top = vec3(0.62, 0.68, 0.80);",
        "  vec3 horizon = vec3(0.30, 0.31, 0.36);",
        "  vec3 ground = vec3(0.07, 0.07, 0.09);",
        "  float y = d.y;",
        "  vec3 sharp = y > 0.0 ? mix(horizon, top, pow(y, 0.6)) : mix(horizon, ground, pow(-y, 0.4));",
        "  vec3 soft = mix(ground, top, 0.5 + 0.5 * y) * 0.7;",
        "  return mix(sharp, soft, rough);",
        "}",
        "vec3 perturb(vec3 n, vec3 p, vec2 uv) {",
        "  vec3 dp1 = dFdx(p);",
        "  vec3 dp2 = dFdy(p);",
        "  vec2 duv1 = dFdx(uv);",
        "  vec2 duv2 = dFdy(uv);",
        "  vec3 dp2perp = cross(dp2, n);",
        "  vec3 dp1perp = cross(n, dp1);",
        "  vec3 t = dp2perp * duv1.x + dp1perp * duv2.x;",
        "  vec3 b = dp2perp * duv1.y + dp1perp * duv2.y;",
        "  float scale = inversesqrt(max(max(dot(t, t), dot(b, b)), 1e-20));",
        "  vec3 m = texture(uNormal, uv).xyz * 2.0 - 1.0;",
        "  m.xy *= uNormalScale;",
        "  return normalize(mat3(t * scale, b * scale, n) * m);",
        "}",
        "float ggx(float nh, float a) {",
        "  float a2 = a * a;",
        "  float d = nh * nh * (a2 - 1.0) + 1.0;",
        "  return a2 / (PI * d * d);",
        "}",
        "vec3 light(vec3 n, vec3 v, vec3 l, vec3 radiance, vec3 diffuse, vec3 f0, float rough) {",
        "  vec3 h = normalize(l + v);",
        "  float nl = max(dot(n, l), 0.0);",
        "  float nv = max(dot(n, v), 1e-3);",
        "  float a = rough * rough;",
        "  float k = (rough + 1.0) * (rough + 1.0) / 8.0;",
        "  float g = nv / (nv * (1.0 - k) + k) * nl / (nl * (1.0 - k) + k);",
        "  vec3 f = f0 + (1.0 - f0) * pow(1.0 - max(dot(h, v), 0.0), 5.0);",
        "  vec3 spec = ggx(max(dot(n, h), 0.0), a) * g * f / max(4.0 * nv * nl, 1e-3);",
        "  return ((1.0 - f) * diffuse / PI + spec) * radiance * nl;",
        "}",
        "void main() {",
        "  vec4 base = texture(uBase, vUv) * uBaseFactor;",
        "  vec4 mr = texture(uMetalRough, vUv);",
        "  float rough = clamp(mr.g * uRoughness, 0.05, 1.0);",
        "  float metal = clamp(mr.b * uMetallic, 0.0, 1.0);",
        "  vec3 n = normalize(vNormal);",
        "  if (!gl_FrontFacing) n = -n;",
        "  n = perturb(n, vPosition, vUv);",
        "  vec3 v = normalize(uCamera - vPosition);",
        "  vec3 diffuse = base.rgb * (1.0 - metal);",
        "  vec3 f0 = mix(vec3(0.04), base.rgb, metal);",
        "  vec3 color = light(n, v, uKey, vec3(3.2, 3.05, 2.85), diffuse, f0, rough);",
        "  color += light(n, v, uRim, vec3(0.9, 1.0, 1.25), diffuse, f0, rough);",
        "  vec3 fenv = f0 + (max(vec3(1.0 - rough), f0) - f0) * pow(1.0 - max(dot(n, v), 0.0), 5.0);",
        "  color += diffuse * sky(n, 1.0) * 0.55;",
        "  color += fenv * sky(reflect(-v, n), rough) * (1.0 - 0.6 * rough);",
        "  color = color / (color + vec3(0.8));",
        "  outColor = vec4(pow(color * 1.45, vec3(1.0 / 2.2)), 1.0);",
        "}"
    ].join("\n");

    // ---- Small matrix helpers (column-major, as WebGL expects) ----

    function identity() {
        return [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
    }

    function multiply(a, b) {
        var out = new Array(16);
        for (var col = 0; col < 4; col++) {
            for (var row = 0; row < 4; row++) {
                var sum = 0;
                for (var k = 0; k < 4; k++) sum += a[k * 4 + row] * b[col * 4 + k];
                out[col * 4 + row] = sum;
            }
        }
        return out;
    }

    function fromTrs(node) {
        if (node.matrix) return node.matrix.slice();
        var t = node.translation || [0, 0, 0];
        var q = node.rotation || [0, 0, 0, 1];
        var s = node.scale || [1, 1, 1];
        var x = q[0], y = q[1], z = q[2], w = q[3];
        return [
            (1 - 2 * (y * y + z * z)) * s[0], 2 * (x * y + z * w) * s[0], 2 * (x * z - y * w) * s[0], 0,
            2 * (x * y - z * w) * s[1], (1 - 2 * (x * x + z * z)) * s[1], 2 * (y * z + x * w) * s[1], 0,
            2 * (x * z + y * w) * s[2], 2 * (y * z - x * w) * s[2], (1 - 2 * (x * x + y * y)) * s[2], 0,
            t[0], t[1], t[2], 1
        ];
    }

    function transformPoint(m, p) {
        return [
            m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12],
            m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13],
            m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14]
        ];
    }

    // Inverse transpose of the upper 3x3, for transforming normals.
    function normalMatrix(m) {
        var a = m[0], b = m[1], c = m[2], d = m[4], e = m[5], f = m[6], g = m[8], h = m[9], i = m[10];
        var A = e * i - f * h, B = f * g - d * i, C = d * h - e * g;
        var det = a * A + b * B + c * C || 1;
        return [
            A / det, B / det, C / det,
            (c * h - b * i) / det, (a * i - c * g) / det, (b * g - a * h) / det,
            (b * f - c * e) / det, (c * d - a * f) / det, (a * e - b * d) / det
        ];
    }

    function perspective(aspect, near, far) {
        var f = 1 / Math.tan(FOV / 2);
        return [f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) / (near - far), -1, 0, 0, 2 * far * near / (near - far), 0];
    }

    function lookAt(eye, target) {
        var zx = eye[0] - target[0], zy = eye[1] - target[1], zz = eye[2] - target[2];
        var zl = Math.hypot(zx, zy, zz);
        zx /= zl; zy /= zl; zz /= zl;
        var xx = zz, xy = 0, xz = -zx;
        var xl = Math.hypot(xx, xz) || 1;
        xx /= xl; xz /= xl;
        var yx = zy * xz - zz * xy, yy = zz * xx - zx * xz, yz = zx * xy - zy * xx;
        return [
            xx, yx, zx, 0,
            xy, yy, zy, 0,
            xz, yz, zz, 0,
            -(xx * eye[0] + xy * eye[1] + xz * eye[2]),
            -(yx * eye[0] + yy * eye[1] + yz * eye[2]),
            -(zx * eye[0] + zy * eye[1] + zz * eye[2]), 1
        ];
    }

    // ---- glTF binary parsing ----

    var COMPONENTS = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4 };

    function parseGlb(buffer) {
        var view = new DataView(buffer);
        if (buffer.byteLength < 28 || view.getUint32(0, true) !== 0x46546c67 || view.getUint32(4, true) !== 2) {
            throw new Error("not a glTF 2.0 binary file");
        }
        var jsonLength = view.getUint32(12, true);
        if (view.getUint32(16, true) !== 0x4e4f534a) throw new Error("missing JSON chunk");
        var json = JSON.parse(new TextDecoder().decode(new Uint8Array(buffer, 20, jsonLength)));
        var binStart = 20 + jsonLength;
        if (binStart + 8 > buffer.byteLength || view.getUint32(binStart + 4, true) !== 0x004e4942) {
            throw new Error("missing binary chunk");
        }
        var binLength = view.getUint32(binStart, true);
        return { json: json, bin: new Uint8Array(buffer, binStart + 8, binLength) };
    }

    function bufferViewBytes(glb, index) {
        var bufferView = glb.json.bufferViews[index];
        return glb.bin.subarray(bufferView.byteOffset || 0, (bufferView.byteOffset || 0) + bufferView.byteLength);
    }

    function readAccessor(glb, index) {
        var accessor = glb.json.accessors[index];
        var size = COMPONENTS[accessor.type];
        var bufferView = glb.json.bufferViews[accessor.bufferView];
        var offset = glb.bin.byteOffset + (bufferView.byteOffset || 0) + (accessor.byteOffset || 0);
        var count = accessor.count * size;
        var types = { 5121: Uint8Array, 5123: Uint16Array, 5125: Uint32Array, 5126: Float32Array };
        var Type = types[accessor.componentType];
        if (!Type || !size) throw new Error("unsupported accessor layout");
        var stride = bufferView.byteStride || 0;
        if (stride && stride !== size * Type.BYTES_PER_ELEMENT) {
            var packed = new Type(count);
            var source = new DataView(glb.bin.buffer);
            for (var i = 0; i < accessor.count; i++) {
                for (var j = 0; j < size; j++) {
                    var at = offset + i * stride + j * Type.BYTES_PER_ELEMENT;
                    packed[i * size + j] = Type === Float32Array ? source.getFloat32(at, true)
                        : Type === Uint32Array ? source.getUint32(at, true)
                        : Type === Uint16Array ? source.getUint16(at, true) : source.getUint8(at);
                }
            }
            return { data: packed, accessor: accessor };
        }
        return { data: new Type(glb.bin.buffer.slice(offset, offset + count * Type.BYTES_PER_ELEMENT)), accessor: accessor };
    }

    function textureSource(glb, info) {
        if (!info) return null;
        var texture = glb.json.textures[info.index];
        var webp = texture.extensions && texture.extensions.EXT_texture_webp;
        var image = glb.json.images[webp ? webp.source : texture.source];
        if (image.bufferView === undefined) throw new Error("external images are not supported");
        return new Blob([bufferViewBytes(glb, image.bufferView)], { type: image.mimeType });
    }

    // ---- Viewer ----

    function Viewer(root) {
        this.root = root;
        this.stage = root.querySelector(".asset-stage");
        this.canvas = root.querySelector(".asset-canvas");
        this.poster = root.querySelector(".asset-poster");
        this.start = root.querySelector(".asset-start");
        this.status = root.querySelector(".asset-status");
        this.reset = root.querySelector(".asset-reset");
        this.models = Array.prototype.slice.call(root.querySelectorAll("[data-asset-model]"));
        this.variants = Array.prototype.slice.call(root.querySelectorAll("[data-asset-variant]"));
        this.downloads = Array.prototype.slice.call(root.querySelectorAll(".asset-download"));
        this.model = this.models.length ? this.models[0].dataset.assetModel : "";
        this.variant = 0;
        this.cache = new Map();
        this.request = null;
        this.generation = 0;
        this.gl = null;
        this.scene = null;
        this.pointers = new Map();
        this.frameRequested = false;
        this.bind();
        this.sync();
    }

    Viewer.prototype.setStatus = function (message, isError) {
        this.status.textContent = message;
        this.status.setAttribute("role", isError ? "alert" : "status");
        this.root.classList.toggle("asset-error", Boolean(isError));
    };

    Viewer.prototype.current = function () {
        var self = this;
        return this.downloads.filter(function (link) {
            return link.dataset.model === self.model && Number(link.dataset.variant) === self.variant;
        })[0];
    };

    // Pressed states, labels and the download link follow the selection.
    Viewer.prototype.sync = function () {
        var self = this;
        this.models.forEach(function (button) {
            button.setAttribute("aria-pressed", String(button.dataset.assetModel === self.model));
        });
        this.variants.forEach(function (button) {
            button.setAttribute("aria-pressed", String(Number(button.dataset.assetVariant) === self.variant));
        });
        var link = this.current();
        this.downloads.forEach(function (item) { item.hidden = item !== link; });
    };

    Viewer.prototype.bind = function () {
        var self = this;
        this.start.addEventListener("click", function () { self.activate(); });
        this.models.forEach(function (button) {
            button.addEventListener("click", function () {
                self.model = button.dataset.assetModel;
                self.variant = 0;
                self.sync();
                if (self.gl) self.load(true);
            });
        });
        this.variants.forEach(function (button) {
            button.addEventListener("click", function () {
                self.variant = Number(button.dataset.assetVariant);
                self.sync();
                if (self.gl) self.load(false);
            });
        });
        this.reset.addEventListener("click", function () {
            if (!self.scene) return;
            self.resetView();
            self.draw();
        });
        this.canvas.addEventListener("pointerdown", function (event) {
            self.canvas.focus({ preventScroll: true });
            self.canvas.setPointerCapture(event.pointerId);
            self.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
        });
        this.canvas.addEventListener("pointermove", function (event) { self.onPointerMove(event); });
        ["pointerup", "pointercancel", "lostpointercapture"].forEach(function (type) {
            self.canvas.addEventListener(type, function (event) { self.pointers.delete(event.pointerId); });
        });
        // The wheel zooms only while the canvas has focus, so scrolling past
        // the viewer keeps scrolling the page.
        this.canvas.addEventListener("wheel", function (event) {
            if (!self.scene || document.activeElement !== self.canvas) return;
            event.preventDefault();
            self.zoom(Math.exp(event.deltaY * 0.0012));
        }, { passive: false });
        this.canvas.addEventListener("keydown", function (event) { self.onKey(event); });
        this.canvas.addEventListener("webglcontextlost", function (event) {
            event.preventDefault();
            self.teardown();
            self.setStatus("The 3D view stopped because the graphics context was lost. Press “Open 3D viewer” to reload it.", true);
        });
        if (window.ResizeObserver) {
            new ResizeObserver(function () { if (self.gl) self.draw(); }).observe(this.stage);
        }
    };

    Viewer.prototype.activate = function () {
        var gl = this.canvas.getContext("webgl2", { antialias: true, alpha: true, premultipliedAlpha: true });
        if (!gl) {
            this.setStatus("This browser cannot show the 3D viewer because WebGL 2 is unavailable. The image and the GLB downloads still work.", true);
            return;
        }
        try {
            this.program = this.createProgram(gl);
        } catch (error) {
            this.setStatus("The 3D viewer could not start: " + error.message, true);
            return;
        }
        this.gl = gl;
        this.anisotropy = gl.getExtension("EXT_texture_filter_anisotropic");
        this.fallback = {
            white: this.solidTexture([255, 255, 255, 255]),
            normal: this.solidTexture([128, 128, 255, 255])
        };
        this.root.classList.add("asset-active");
        this.start.hidden = true;
        this.canvas.hidden = false;
        this.reset.disabled = false;
        this.canvas.focus({ preventScroll: true });
        this.load(true);
    };

    Viewer.prototype.teardown = function () {
        this.generation += 1;
        if (this.request) this.request.abort();
        this.request = null;
        this.gl = null;
        this.scene = null;
        this.root.classList.remove("asset-active");
        this.canvas.hidden = true;
        this.start.hidden = false;
        this.reset.disabled = true;
    };

    Viewer.prototype.createProgram = function (gl) {
        function compile(type, source) {
            var shader = gl.createShader(type);
            gl.shaderSource(shader, source);
            gl.compileShader(shader);
            if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS) && !gl.isContextLost()) {
                throw new Error(gl.getShaderInfoLog(shader) || "shader compilation failed");
            }
            return shader;
        }
        var program = gl.createProgram();
        gl.attachShader(program, compile(gl.VERTEX_SHADER, VERTEX_SHADER));
        gl.attachShader(program, compile(gl.FRAGMENT_SHADER, FRAGMENT_SHADER));
        gl.bindAttribLocation(program, 0, "inPosition");
        gl.bindAttribLocation(program, 1, "inNormal");
        gl.bindAttribLocation(program, 2, "inUv");
        gl.linkProgram(program);
        if (!gl.getProgramParameter(program, gl.LINK_STATUS) && !gl.isContextLost()) {
            throw new Error(gl.getProgramInfoLog(program) || "shader linking failed");
        }
        var uniforms = {};
        ["uModel", "uViewProjection", "uNormalMatrix", "uBase", "uNormal", "uMetalRough", "uBaseFactor",
            "uMetallic", "uRoughness", "uNormalScale", "uCamera", "uKey", "uRim"].forEach(function (name) {
            uniforms[name] = gl.getUniformLocation(program, name);
        });
        return { handle: program, uniforms: uniforms };
    };

    Viewer.prototype.solidTexture = function (rgba) {
        var gl = this.gl;
        var texture = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array(rgba));
        return texture;
    };

    Viewer.prototype.uploadTexture = function (bitmap, srgb) {
        var gl = this.gl;
        var texture = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false);
        gl.texImage2D(gl.TEXTURE_2D, 0, srgb ? gl.SRGB8_ALPHA8 : gl.RGBA8, gl.RGBA, gl.UNSIGNED_BYTE, bitmap);
        gl.generateMipmap(gl.TEXTURE_2D);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.REPEAT);
        if (this.anisotropy) gl.texParameterf(gl.TEXTURE_2D, this.anisotropy.TEXTURE_MAX_ANISOTROPY_EXT, 8);
        if (bitmap.close) bitmap.close();
        return texture;
    };

    // Fetches (or reuses) the selected file and replaces the scene once it is
    // ready. A newer selection invalidates older requests.
    Viewer.prototype.load = function (resetCamera) {
        var self = this;
        var link = this.current();
        if (!link) return;
        var generation = ++this.generation;
        if (this.request) this.request.abort();
        var label = link.dataset.label;
        this.setStatus("Loading " + label + "…", false);
        this.root.setAttribute("aria-busy", "true");
        var cached = this.cache.get(link.getAttribute("href"));
        var pending;
        if (cached) {
            pending = Promise.resolve(cached);
        } else {
            this.request = new AbortController();
            pending = window.fetch(link.getAttribute("href"), { signal: this.request.signal }).then(function (response) {
                if (!response.ok) throw new Error("HTTP " + response.status);
                return response.arrayBuffer();
            });
        }
        pending.then(function (buffer) {
            if (generation !== self.generation) return null;
            self.cache.set(link.getAttribute("href"), buffer);
            return self.buildScene(parseGlb(buffer)).then(function (scene) {
                if (generation !== self.generation || !self.gl) {
                    self.dispose(scene);
                    return;
                }
                self.dispose(self.scene);
                self.scene = scene;
                if (resetCamera || !self.view) self.resetView();
                else self.view.distance = Math.min(Math.max(self.view.distance, scene.radius * 0.8), scene.radius * 8);
                self.root.removeAttribute("aria-busy");
                self.setStatus(label + ": " + scene.vertices.toLocaleString("en") + " vertices, " +
                    scene.triangles.toLocaleString("en") + " triangles.", false);
                self.draw();
            });
        }).catch(function (error) {
            if (generation !== self.generation || (error && error.name === "AbortError")) return;
            self.root.removeAttribute("aria-busy");
            self.setStatus("Could not load " + label + " (" + error.message + "). Choose it again to retry, or download the GLB file.", true);
        });
    };

    Viewer.prototype.buildScene = function (glb) {
        var self = this;
        var gl = this.gl;
        var json = glb.json;
        var required = json.extensionsRequired || [];
        var unsupported = required.filter(function (name) { return name !== "EXT_texture_webp"; });
        if (unsupported.length) return Promise.reject(new Error("unsupported extension " + unsupported[0]));
        var draws = [];
        var bitmaps = [];
        var resources = [];
        var min = [Infinity, Infinity, Infinity];
        var max = [-Infinity, -Infinity, -Infinity];
        var vertices = 0;
        var triangles = 0;

        function decode(info, srgb) {
            var blob = textureSource(glb, info);
            if (!blob) return Promise.resolve(null);
            return window.createImageBitmap(blob, { colorSpaceConversion: "none", premultiplyAlpha: "none" })
                .then(function (bitmap) { return { bitmap: bitmap, srgb: srgb }; });
        }

        function visit(index, parent) {
            var node = json.nodes[index];
            var matrix = multiply(parent, fromTrs(node));
            if (node.mesh !== undefined) {
                json.meshes[node.mesh].primitives.forEach(function (primitive) {
                    if ((primitive.mode === undefined ? 4 : primitive.mode) !== 4) return;
                    var attributes = primitive.attributes;
                    if (attributes.POSITION === undefined || attributes.NORMAL === undefined) {
                        throw new Error("a mesh lacks positions or normals");
                    }
                    var position = readAccessor(glb, attributes.POSITION);
                    var accessor = position.accessor;
                    for (var corner = 0; corner < 8; corner++) {
                        var p = transformPoint(matrix, [
                            corner & 1 ? accessor.max[0] : accessor.min[0],
                            corner & 2 ? accessor.max[1] : accessor.min[1],
                            corner & 4 ? accessor.max[2] : accessor.min[2]
                        ]);
                        for (var axis = 0; axis < 3; axis++) {
                            min[axis] = Math.min(min[axis], p[axis]);
                            max[axis] = Math.max(max[axis], p[axis]);
                        }
                    }
                    var vao = gl.createVertexArray();
                    resources.push(vao);
                    gl.bindVertexArray(vao);
                    [[attributes.POSITION, 0, 3], [attributes.NORMAL, 1, 3], [attributes.TEXCOORD_0, 2, 2]].forEach(function (entry) {
                        if (entry[0] === undefined) {
                            gl.disableVertexAttribArray(entry[1]);
                            gl.vertexAttrib2f(entry[1], 0, 0);
                            return;
                        }
                        var values = entry[0] === attributes.POSITION ? position : readAccessor(glb, entry[0]);
                        if (!(values.data instanceof Float32Array)) throw new Error("vertex attributes must be floats");
                        var buffer = gl.createBuffer();
                        resources.push(buffer);
                        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
                        gl.bufferData(gl.ARRAY_BUFFER, values.data, gl.STATIC_DRAW);
                        gl.enableVertexAttribArray(entry[1]);
                        gl.vertexAttribPointer(entry[1], entry[2], gl.FLOAT, false, 0, 0);
                    });
                    var count = accessor.count;
                    var indexType = null;
                    if (primitive.indices !== undefined) {
                        var indices = readAccessor(glb, primitive.indices);
                        var indexBuffer = gl.createBuffer();
                        resources.push(indexBuffer);
                        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
                        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices.data, gl.STATIC_DRAW);
                        count = indices.accessor.count;
                        indexType = indices.data instanceof Uint32Array ? gl.UNSIGNED_INT
                            : indices.data instanceof Uint16Array ? gl.UNSIGNED_SHORT : gl.UNSIGNED_BYTE;
                    }
                    gl.bindVertexArray(null);
                    vertices += accessor.count;
                    triangles += Math.floor(count / 3);
                    var material = (json.materials || [])[primitive.material] || {};
                    var pbr = material.pbrMetallicRoughness || {};
                    var draw = {
                        vao: vao, count: count, indexType: indexType, matrix: matrix,
                        baseFactor: pbr.baseColorFactor || [1, 1, 1, 1],
                        metallic: pbr.metallicFactor === undefined ? 1 : pbr.metallicFactor,
                        roughness: pbr.roughnessFactor === undefined ? 1 : pbr.roughnessFactor,
                        normalScale: material.normalTexture && material.normalTexture.scale !== undefined ? material.normalTexture.scale : 1,
                        textures: {}
                    };
                    bitmaps.push(decode(pbr.baseColorTexture, true).then(function (t) { draw.textures.base = t; }));
                    bitmaps.push(decode(material.normalTexture, false).then(function (t) { draw.textures.normal = t; }));
                    bitmaps.push(decode(pbr.metallicRoughnessTexture, false).then(function (t) { draw.textures.metalRough = t; }));
                    draws.push(draw);
                });
            }
            (node.children || []).forEach(function (child) { visit(child, matrix); });
        }

        try {
            var scene = json.scenes[json.scene || 0];
            scene.nodes.forEach(function (index) { visit(index, identity()); });
        } catch (error) {
            return Promise.reject(error);
        }
        if (!draws.length) return Promise.reject(new Error("the file contains no triangle mesh"));
        return Promise.all(bitmaps).then(function () {
            if (!self.gl) return null;
            draws.forEach(function (draw) {
                ["base", "normal", "metalRough"].forEach(function (key) {
                    var item = draw.textures[key];
                    draw.textures[key] = item ? self.uploadTexture(item.bitmap, item.srgb)
                        : (key === "normal" ? self.fallback.normal : self.fallback.white);
                    resources.push(draw.textures[key]);
                });
            });
            var center = [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2];
            var radius = Math.hypot(max[0] - min[0], max[1] - min[1], max[2] - min[2]) / 2 || 1;
            return { draws: draws, resources: resources, center: center, radius: radius, vertices: vertices, triangles: triangles };
        });
    };

    // Frees the GPU objects of a replaced scene; the fallback textures stay.
    Viewer.prototype.dispose = function (scene) {
        var gl = this.gl;
        if (!gl || !scene) return;
        var fallback = this.fallback;
        scene.resources.forEach(function (item) {
            if (item instanceof WebGLVertexArrayObject) gl.deleteVertexArray(item);
            else if (item instanceof WebGLBuffer) gl.deleteBuffer(item);
            else if (item instanceof WebGLTexture && item !== fallback.white && item !== fallback.normal) gl.deleteTexture(item);
        });
    };

    Viewer.prototype.resetView = function () {
        this.view = { yaw: 0.65, pitch: 0.42, distance: this.scene.radius / Math.sin(FOV / 2) * 0.85 };
    };

    Viewer.prototype.zoom = function (factor) {
        var radius = this.scene.radius;
        this.view.distance = Math.min(Math.max(this.view.distance * factor, radius * 0.8), radius * 8);
        this.draw();
    };

    Viewer.prototype.orbit = function (dx, dy) {
        this.view.yaw -= dx;
        this.view.pitch = Math.min(Math.max(this.view.pitch + dy, MIN_PITCH), MAX_PITCH);
        this.draw();
    };

    Viewer.prototype.onPointerMove = function (event) {
        var previous = this.pointers.get(event.pointerId);
        if (!previous || !this.scene) return;
        var points = Array.from(this.pointers.values());
        if (this.pointers.size === 2) {
            var other = points[0] === previous ? points[1] : points[0];
            var before = Math.hypot(previous.x - other.x, previous.y - other.y);
            var after = Math.hypot(event.clientX - other.x, event.clientY - other.y);
            if (before > 0 && after > 0) this.zoom(before / after);
        } else if (this.pointers.size === 1) {
            this.orbit((event.clientX - previous.x) * 0.008, (event.clientY - previous.y) * 0.008);
        }
        this.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
    };

    Viewer.prototype.onKey = function (event) {
        if (!this.scene) return;
        var step = 0.12;
        var handled = true;
        switch (event.key) {
            case "ArrowLeft": this.orbit(-step, 0); break;
            case "ArrowRight": this.orbit(step, 0); break;
            case "ArrowUp": this.orbit(0, step); break;
            case "ArrowDown": this.orbit(0, -step); break;
            case "+": case "=": this.zoom(0.85); break;
            case "-": case "_": this.zoom(1 / 0.85); break;
            case "0": case "Home": this.resetView(); this.draw(); break;
            default: handled = false;
        }
        if (handled) event.preventDefault();
    };

    // Drawing happens only after a change, at most once per animation frame.
    Viewer.prototype.draw = function () {
        var self = this;
        if (this.frameRequested) return;
        this.frameRequested = true;
        window.requestAnimationFrame(function () {
            self.frameRequested = false;
            self.render();
        });
    };

    Viewer.prototype.render = function () {
        var gl = this.gl;
        var scene = this.scene;
        if (!gl || !scene || gl.isContextLost()) return;
        var ratio = Math.min(window.devicePixelRatio || 1, 2);
        var width = Math.max(1, Math.round(this.canvas.clientWidth * ratio));
        var height = Math.max(1, Math.round(this.canvas.clientHeight * ratio));
        if (this.canvas.width !== width || this.canvas.height !== height) {
            this.canvas.width = width;
            this.canvas.height = height;
        }
        gl.viewport(0, 0, width, height);
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
        gl.enable(gl.DEPTH_TEST);
        gl.disable(gl.CULL_FACE);

        // glTF is Y-up; Blender's exporter converts SimForge's Z-up scenes.
        var view = this.view;
        var c = scene.center;
        var eye = [
            c[0] + view.distance * Math.cos(view.pitch) * Math.sin(view.yaw),
            c[1] + view.distance * Math.sin(view.pitch),
            c[2] + view.distance * Math.cos(view.pitch) * Math.cos(view.yaw)
        ];
        var near = Math.max(view.distance - scene.radius * 1.5, view.distance * 0.01);
        var far = view.distance + scene.radius * 1.5;
        var viewProjection = multiply(perspective(width / height, near, far), lookAt(eye, c));
        var program = this.program;
        var u = program.uniforms;
        gl.useProgram(program.handle);
        gl.uniformMatrix4fv(u.uViewProjection, false, viewProjection);
        gl.uniform3fv(u.uCamera, eye);
        // Lights follow the camera so the asset stays readable from any side.
        var key = [Math.sin(view.yaw + 0.9) * 0.6, 0.75, Math.cos(view.yaw + 0.9) * 0.6];
        var rim = [-Math.sin(view.yaw - 0.4) * 0.7, 0.35, -Math.cos(view.yaw - 0.4) * 0.7];
        gl.uniform3fv(u.uKey, normalize(key));
        gl.uniform3fv(u.uRim, normalize(rim));
        gl.uniform1i(u.uBase, 0);
        gl.uniform1i(u.uNormal, 1);
        gl.uniform1i(u.uMetalRough, 2);
        scene.draws.forEach(function (draw) {
            gl.uniformMatrix4fv(u.uModel, false, draw.matrix);
            gl.uniformMatrix3fv(u.uNormalMatrix, false, normalMatrix(draw.matrix));
            gl.uniform4fv(u.uBaseFactor, draw.baseFactor);
            gl.uniform1f(u.uMetallic, draw.metallic);
            gl.uniform1f(u.uRoughness, draw.roughness);
            gl.uniform1f(u.uNormalScale, draw.normalScale);
            [draw.textures.base, draw.textures.normal, draw.textures.metalRough].forEach(function (texture, unit) {
                gl.activeTexture(gl.TEXTURE0 + unit);
                gl.bindTexture(gl.TEXTURE_2D, texture);
            });
            gl.bindVertexArray(draw.vao);
            if (draw.indexType) gl.drawElements(gl.TRIANGLES, draw.count, draw.indexType, 0);
            else gl.drawArrays(gl.TRIANGLES, 0, draw.count);
        });
        gl.bindVertexArray(null);
    };

    function normalize(v) {
        var length = Math.hypot(v[0], v[1], v[2]) || 1;
        return [v[0] / length, v[1] / length, v[2] / length];
    }

    Array.prototype.forEach.call(document.querySelectorAll("[data-asset-explorer]"), function (root) {
        if (root.dataset.assetExplorerBound === "true") return;
        root.dataset.assetExplorerBound = "true";
        new Viewer(root);
    });
})();
