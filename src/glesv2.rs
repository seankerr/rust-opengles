// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0(the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License Author                                                   |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@code-box.org>                                                         |
// +-----------------------------------------------------------------------------------------------+

#![allow(dead_code)]

// -------------------------------------------------------------------------------------------------
// LINKING
// -------------------------------------------------------------------------------------------------

#[link(name = "GLESv2")]
extern {}

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

use khronos::{ khronos_float_t,
               khronos_int8_t,
               khronos_int32_t,
               khronos_intptr_t,
               khronos_ssize_t,
               khronos_uint8_t };

use libc::{ c_char,
            c_int,
            c_short,
            c_uchar,
            c_uint,
            c_ushort,
            c_void };

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------

pub type GLbitfield = c_uint;
pub type GLboolean  = c_uchar;
pub type GLbyte     = khronos_int8_t;
pub type GLchar     = c_char;
pub type GLclampf   = khronos_float_t;
pub type GLenum     = c_uint;
pub type GLfixed    = khronos_int32_t;
pub type GLfloat    = khronos_float_t;
pub type GLint      = c_int;
pub type GLshort    = c_short;
pub type GLsizei    = c_int;
pub type GLubyte    = khronos_uint8_t;
pub type GLuint     = c_uint;
pub type GLushort   = c_ushort;
pub type GLvoid     = c_void;

pub type GLintptr   = khronos_intptr_t;
pub type GLsizeiptr = khronos_ssize_t;

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

// OpenGL ES core versions
pub const GL_ES_VERSION_2_0: GLint = 1;

// ClearBufferMask
pub const GL_DEPTH_BUFFER_BIT:   GLint = 0x00000100;
pub const GL_STENCIL_BUFFER_BIT: GLint = 0x00000400;
pub const GL_COLOR_BUFFER_BIT:   GLint = 0x00004000;

// boolean
pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE:  GLboolean = 1;

// BeginMode
pub const GL_POINTS:         GLint = 0x0000;
pub const GL_LINES:          GLint = 0x0001;
pub const GL_LINE_LOOP:      GLint = 0x0002;
pub const GL_LINE_STRIP:     GLint = 0x0003;
pub const GL_TRIANGLES:      GLint = 0x0004;
pub const GL_TRIANGLE_STRIP: GLint = 0x0005;
pub const GL_TRIANGLE_FAN:   GLint = 0x0006;

// AlphaFunction(not supported in ES20)
//      GL_NEVER
//      GL_LESS
//      GL_EQUAL
//      GL_LEQUAL
//      GL_GREATER
//      GL_NOTEQUAL
//      GL_GEQUAL
//      GL_ALWAYS

// BlendingFactorDest
pub const GL_ZERO:                GLint = 0;
pub const GL_ONE:                 GLint = 1;
pub const GL_SRC_COLOR:           GLint = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLint = 0x0301;
pub const GL_SRC_ALPHA:           GLint = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLint = 0x0303;
pub const GL_DST_ALPHA:           GLint = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLint = 0x0305;

// BlendingFactorSrc
//      GL_ZERO
//      GL_ONE
pub const GL_DST_COLOR:           GLint = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLint = 0x0307;
pub const GL_SRC_ALPHA_SATURATE:  GLint = 0x0308;
//      GL_SRC_ALPHA
//      GL_ONE_MINUS_SRC_ALPHA
//      GL_DST_ALPHA
//      GL_ONE_MINUS_DST_ALPHA

// BlendEquationSeparate
pub const GL_FUNC_ADD:             GLint = 0x8006;
pub const GL_BLEND_EQUATION:       GLint = 0x8009;
pub const GL_BLEND_EQUATION_RGB:   GLint = 0x8009; // same as BLEND_EQUATION
pub const GL_BLEND_EQUATION_ALPHA: GLint = 0x883D;

// BlendSubtract
pub const GL_FUNC_SUBTRACT:         GLint = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: GLint = 0x800B;

// Separate Blend Functions
pub const GL_BLEND_DST_RGB:            GLint = 0x80C8;
pub const GL_BLEND_SRC_RGB:            GLint = 0x80C9;
pub const GL_BLEND_DST_ALPHA:          GLint = 0x80CA;
pub const GL_BLEND_SRC_ALPHA:          GLint = 0x80CB;
pub const GL_CONSTANT_COLOR:           GLint = 0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLint = 0x8002;
pub const GL_CONSTANT_ALPHA:           GLint = 0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLint = 0x8004;
pub const GL_BLEND_COLOR:              GLint = 0x8005;

// Buffer Objects
pub const GL_ARRAY_BUFFER:                 GLint = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER:         GLint = 0x8893;
pub const GL_ARRAY_BUFFER_BINDING:         GLint = 0x8894;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLint = 0x8895;

pub const GL_STREAM_DRAW:  GLint = 0x88E0;
pub const GL_STATIC_DRAW:  GLint = 0x88E4;
pub const GL_DYNAMIC_DRAW: GLint = 0x88E8;

pub const GL_BUFFER_SIZE:  GLint = 0x8764;
pub const GL_BUFFER_USAGE: GLint = 0x8765;

pub const GL_CURRENT_VERTEX_ATTRIB: GLint = 0x8626;

// CullFaceMode
pub const GL_FRONT:          GLint = 0x0404;
pub const GL_BACK:           GLint = 0x0405;
pub const GL_FRONT_AND_BACK: GLint = 0x0408;

// DepthFunction
//      GL_NEVER
//      GL_LESS
//      GL_EQUAL
//      GL_LEQUAL
//      GL_GREATER
//      GL_NOTEQUAL
//      GL_GEQUAL
//      GL_ALWAYS

// EnableCap
pub const GL_TEXTURE_2D:               GLint = 0x0DE1;
pub const GL_CULL_FACE:                GLint = 0x0B44;
pub const GL_BLEND:                    GLint = 0x0BE2;
pub const GL_DITHER:                   GLint = 0x0BD0;
pub const GL_STENCIL_TEST:             GLint = 0x0B90;
pub const GL_DEPTH_TEST:               GLint = 0x0B71;
pub const GL_SCISSOR_TEST:             GLint = 0x0C11;
pub const GL_POLYGON_OFFSET_FILL:      GLint = 0x8037;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLint = 0x809E;
pub const GL_SAMPLE_COVERAGE:          GLint = 0x80A0;

// ErrorCode
pub const GL_NO_ERROR:          GLint = 0;
pub const GL_INVALID_ENUM:      GLint = 0x0500;
pub const GL_INVALID_VALUE:     GLint = 0x0501;
pub const GL_INVALID_OPERATION: GLint = 0x0502;
pub const GL_OUT_OF_MEMORY:     GLint = 0x0505;

// FrontFaceDirection
pub const GL_CW:  GLint = 0x0900;
pub const GL_CCW: GLint = 0x0901;

// GetPName
pub const GL_LINE_WIDTH:                   GLint = 0x0B21;
pub const GL_ALIASED_POINT_SIZE_RANGE:     GLint = 0x846D;
pub const GL_ALIASED_LINE_WIDTH_RANGE:     GLint = 0x846E;
pub const GL_CULL_FACE_MODE:               GLint = 0x0B45;
pub const GL_FRONT_FACE:                   GLint = 0x0B46;
pub const GL_DEPTH_RANGE:                  GLint = 0x0B70;
pub const GL_DEPTH_WRITEMASK:              GLint = 0x0B72;
pub const GL_DEPTH_CLEAR_VALUE:            GLint = 0x0B73;
pub const GL_DEPTH_FUNC:                   GLint = 0x0B74;
pub const GL_STENCIL_CLEAR_VALUE:          GLint = 0x0B91;
pub const GL_STENCIL_FUNC:                 GLint = 0x0B92;
pub const GL_STENCIL_FAIL:                 GLint = 0x0B94;
pub const GL_STENCIL_PASS_DEPTH_FAIL:      GLint = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS:      GLint = 0x0B96;
pub const GL_STENCIL_REF:                  GLint = 0x0B97;
pub const GL_STENCIL_VALUE_MASK:           GLint = 0x0B93;
pub const GL_STENCIL_WRITEMASK:            GLint = 0x0B98;
pub const GL_STENCIL_BACK_FUNC:            GLint = 0x8800;
pub const GL_STENCIL_BACK_FAIL:            GLint = 0x8801;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLint = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLint = 0x8803;
pub const GL_STENCIL_BACK_REF:             GLint = 0x8CA3;
pub const GL_STENCIL_BACK_VALUE_MASK:      GLint = 0x8CA4;
pub const GL_STENCIL_BACK_WRITEMASK:       GLint = 0x8CA5;
pub const GL_VIEWPORT:                     GLint = 0x0BA2;
pub const GL_SCISSOR_BOX:                  GLint = 0x0C10;
//      GL_SCISSOR_TEST
pub const GL_COLOR_CLEAR_VALUE:            GLint = 0x0C22;
pub const GL_COLOR_WRITEMASK:              GLint = 0x0C23;
pub const GL_UNPACK_ALIGNMENT:             GLint = 0x0CF5;
pub const GL_PACK_ALIGNMENT:               GLint = 0x0D05;
pub const GL_MAX_TEXTURE_SIZE:             GLint = 0x0D33;
pub const GL_MAX_VIEWPORT_DIMS:            GLint = 0x0D3A;
pub const GL_SUBPIXEL_BITS:                GLint = 0x0D50;
pub const GL_RED_BITS:                     GLint = 0x0D52;
pub const GL_GREEN_BITS:                   GLint = 0x0D53;
pub const GL_BLUE_BITS:                    GLint = 0x0D54;
pub const GL_ALPHA_BITS:                   GLint = 0x0D55;
pub const GL_DEPTH_BITS:                   GLint = 0x0D56;
pub const GL_STENCIL_BITS:                 GLint = 0x0D57;
pub const GL_POLYGON_OFFSET_UNITS:         GLint = 0x2A00;
//      GL_POLYGON_OFFSET_FILL
pub const GL_POLYGON_OFFSET_FACTOR:        GLint = 0x8038;
pub const GL_TEXTURE_BINDING_2D:           GLint = 0x8069;
pub const GL_SAMPLE_BUFFERS:               GLint = 0x80A8;
pub const GL_SAMPLES:                      GLint = 0x80A9;
pub const GL_SAMPLE_COVERAGE_VALUE:        GLint = 0x80AA;
pub const GL_SAMPLE_COVERAGE_INVERT:       GLint = 0x80AB;

// GetTextureParameter
//      GL_TEXTURE_MAG_FILTER
//      GL_TEXTURE_MIN_FILTER
//      GL_TEXTURE_WRAP_S
//      GL_TEXTURE_WRAP_T

pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLint = 0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS:     GLint = 0x86A3;

// HintMode
pub const GL_DONT_CARE: GLint = 0x1100;
pub const GL_FASTEST:   GLint = 0x1101;
pub const GL_NICEST:    GLint = 0x1102;

// HintTarget
pub const GL_GENERATE_MIPMAP_HINT: GLint = 0x8192;

// DataType
pub const GL_BYTE:           GLint = 0x1400;
pub const GL_UNSIGNED_BYTE:  GLint = 0x1401;
pub const GL_SHORT:          GLint = 0x1402;
pub const GL_UNSIGNED_SHORT: GLint = 0x1403;
pub const GL_INT:            GLint = 0x1404;
pub const GL_UNSIGNED_INT:   GLint = 0x1405;
pub const GL_FLOAT:          GLint = 0x1406;
pub const GL_FIXED:          GLint = 0x140C;

// PixelFormat
pub const GL_DEPTH_COMPONENT: GLint = 0x1902;
pub const GL_ALPHA:           GLint = 0x1906;
pub const GL_RGB:             GLint = 0x1907;
pub const GL_RGBA:            GLint = 0x1908;
pub const GL_LUMINANCE:       GLint = 0x1909;
pub const GL_LUMINANCE_ALPHA: GLint = 0x190A;

// PixelType
//      GL_UNSIGNED_BYTE
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLint = 0x8033;
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLint = 0x8034;
pub const GL_UNSIGNED_SHORT_5_6_5:   GLint = 0x8363;

// Shaders
pub const GL_FRAGMENT_SHADER:                  GLint = 0x8B30;
pub const GL_VERTEX_SHADER:                    GLint = 0x8B31;
pub const GL_MAX_VERTEX_ATTRIBS:               GLint = 0x8869;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS:       GLint = 0x8DFB;
pub const GL_MAX_VARYING_VECTORS:              GLint = 0x8DFC;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLint = 0x8B4D;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS:   GLint = 0x8B4C;
pub const GL_MAX_TEXTURE_IMAGE_UNITS:          GLint = 0x8872;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS:     GLint = 0x8DFD;
pub const GL_SHADER_TYPE:                      GLint = 0x8B4F;
pub const GL_DELETE_STATUS:                    GLint = 0x8B80;
pub const GL_LINK_STATUS:                      GLint = 0x8B82;
pub const GL_VALIDATE_STATUS:                  GLint = 0x8B83;
pub const GL_ATTACHED_SHADERS:                 GLint = 0x8B85;
pub const GL_ACTIVE_UNIFORMS:                  GLint = 0x8B86;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH:        GLint = 0x8B87;
pub const GL_ACTIVE_ATTRIBUTES:                GLint = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH:      GLint = 0x8B8A;
pub const GL_SHADING_LANGUAGE_VERSION:         GLint = 0x8B8C;
pub const GL_CURRENT_PROGRAM:                  GLint = 0x8B8D;

// StencilFunction
pub const GL_NEVER:    GLint = 0x0200;
pub const GL_LESS:     GLint = 0x0201;
pub const GL_EQUAL:    GLint = 0x0202;
pub const GL_LEQUAL:   GLint = 0x0203;
pub const GL_GREATER:  GLint = 0x0204;
pub const GL_NOTEQUAL: GLint = 0x0205;
pub const GL_GEQUAL:   GLint = 0x0206;
pub const GL_ALWAYS:   GLint = 0x0207;

// StencilOp
//      GL_ZERO
pub const GL_KEEP:      GLint = 0x1E00;
pub const GL_REPLACE:   GLint = 0x1E01;
pub const GL_INCR:      GLint = 0x1E02;
pub const GL_DECR:      GLint = 0x1E03;
pub const GL_INVERT:    GLint = 0x150A;
pub const GL_INCR_WRAP: GLint = 0x8507;
pub const GL_DECR_WRAP: GLint = 0x8508;

// StringName
pub const GL_VENDOR:     GLint = 0x1F00;
pub const GL_RENDERER:   GLint = 0x1F01;
pub const GL_VERSION:    GLint = 0x1F02;
pub const GL_EXTENSIONS: GLint = 0x1F03;

// TextureMagFilter
pub const GL_NEAREST: GLint = 0x2600;
pub const GL_LINEAR:  GLint = 0x2601;

// TextureMinFilter
//      GL_NEAREST
//      GL_LINEAR
pub const GL_NEAREST_MIPMAP_NEAREST: GLint = 0x2700;
pub const GL_LINEAR_MIPMAP_NEAREST:  GLint = 0x2701;
pub const GL_NEAREST_MIPMAP_LINEAR:  GLint = 0x2702;
pub const GL_LINEAR_MIPMAP_LINEAR:   GLint = 0x2703;

// TextureParameterName
pub const GL_TEXTURE_MAG_FILTER: GLint = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: GLint = 0x2801;
pub const GL_TEXTURE_WRAP_S:     GLint = 0x2802;
pub const GL_TEXTURE_WRAP_T:     GLint = 0x2803;

// TextureTarget
//      GL_TEXTURE_2D
pub const GL_TEXTURE:                     GLint = 0x1702;
pub const GL_TEXTURE_CUBE_MAP:            GLint = 0x8513;
pub const GL_TEXTURE_BINDING_CUBE_MAP:    GLint = 0x8514;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLint = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLint = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLint = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLint = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLint = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLint = 0x851A;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE:   GLint = 0x851C;

// TextureUnit
pub const GL_TEXTURE0:       GLint = 0x84C0;
pub const GL_TEXTURE1:       GLint = 0x84C1;
pub const GL_TEXTURE2:       GLint = 0x84C2;
pub const GL_TEXTURE3:       GLint = 0x84C3;
pub const GL_TEXTURE4:       GLint = 0x84C4;
pub const GL_TEXTURE5:       GLint = 0x84C5;
pub const GL_TEXTURE6:       GLint = 0x84C6;
pub const GL_TEXTURE7:       GLint = 0x84C7;
pub const GL_TEXTURE8:       GLint = 0x84C8;
pub const GL_TEXTURE9:       GLint = 0x84C9;
pub const GL_TEXTURE10:      GLint = 0x84CA;
pub const GL_TEXTURE11:      GLint = 0x84CB;
pub const GL_TEXTURE12:      GLint = 0x84CC;
pub const GL_TEXTURE13:      GLint = 0x84CD;
pub const GL_TEXTURE14:      GLint = 0x84CE;
pub const GL_TEXTURE15:      GLint = 0x84CF;
pub const GL_TEXTURE16:      GLint = 0x84D0;
pub const GL_TEXTURE17:      GLint = 0x84D1;
pub const GL_TEXTURE18:      GLint = 0x84D2;
pub const GL_TEXTURE19:      GLint = 0x84D3;
pub const GL_TEXTURE20:      GLint = 0x84D4;
pub const GL_TEXTURE21:      GLint = 0x84D5;
pub const GL_TEXTURE22:      GLint = 0x84D6;
pub const GL_TEXTURE23:      GLint = 0x84D7;
pub const GL_TEXTURE24:      GLint = 0x84D8;
pub const GL_TEXTURE25:      GLint = 0x84D9;
pub const GL_TEXTURE26:      GLint = 0x84DA;
pub const GL_TEXTURE27:      GLint = 0x84DB;
pub const GL_TEXTURE28:      GLint = 0x84DC;
pub const GL_TEXTURE29:      GLint = 0x84DD;
pub const GL_TEXTURE30:      GLint = 0x84DE;
pub const GL_TEXTURE31:      GLint = 0x84DF;
pub const GL_ACTIVE_TEXTURE: GLint = 0x84E0;

// TextureWrapMode
pub const GL_REPEAT:          GLint = 0x2901;
pub const GL_CLAMP_TO_EDGE:   GLint = 0x812F;
pub const GL_MIRRORED_REPEAT: GLint = 0x8370;

// Uniform Types
pub const GL_FLOAT_VEC2:   GLint = 0x8B50;
pub const GL_FLOAT_VEC3:   GLint = 0x8B51;
pub const GL_FLOAT_VEC4:   GLint = 0x8B52;
pub const GL_INT_VEC2:     GLint = 0x8B53;
pub const GL_INT_VEC3:     GLint = 0x8B54;
pub const GL_INT_VEC4:     GLint = 0x8B55;
pub const GL_BOOL:         GLint = 0x8B56;
pub const GL_BOOL_VEC2:    GLint = 0x8B57;
pub const GL_BOOL_VEC3:    GLint = 0x8B58;
pub const GL_BOOL_VEC4:    GLint = 0x8B59;
pub const GL_FLOAT_MAT2:   GLint = 0x8B5A;
pub const GL_FLOAT_MAT3:   GLint = 0x8B5B;
pub const GL_FLOAT_MAT4:   GLint = 0x8B5C;
pub const GL_SAMPLER_2D:   GLint = 0x8B5E;
pub const GL_SAMPLER_CUBE: GLint = 0x8B60;

// Vertex Arrays
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED:        GLint = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE:           GLint = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE:         GLint = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE:           GLint = 0x8625;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED:     GLint = 0x886A;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER:        GLint = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLint = 0x889F;

// Read Format
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE:   GLint = 0x8B9A;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLint = 0x8B9B;

// Shader Source
pub const GL_COMPILE_STATUS:       GLint = 0x8B81;
pub const GL_INFO_LOG_LENGTH:      GLint = 0x8B84;
pub const GL_SHADER_SOURCE_LENGTH: GLint = 0x8B88;
pub const GL_SHADER_COMPILER:      GLint = 0x8DFA;

// Shader Binary
pub const GL_SHADER_BINARY_FORMATS:     GLint = 0x8DF8;
pub const GL_NUM_SHADER_BINARY_FORMATS: GLint = 0x8DF9;

// Shader Precision-Specified Types
pub const GL_LOW_FLOAT:    GLint = 0x8DF0;
pub const GL_MEDIUM_FLOAT: GLint = 0x8DF1;
pub const GL_HIGH_FLOAT:   GLint = 0x8DF2;
pub const GL_LOW_INT:      GLint = 0x8DF3;
pub const GL_MEDIUM_INT:   GLint = 0x8DF4;
pub const GL_HIGH_INT:     GLint = 0x8DF5;

// Framebuffer Object.
pub const GL_FRAMEBUFFER:  GLint = 0x8D40;
pub const GL_RENDERBUFFER: GLint = 0x8D41;

pub const GL_RGBA4:             GLint = 0x8056;
pub const GL_RGB5_A1:           GLint = 0x8057;
pub const GL_RGB565:            GLint = 0x8D62;
pub const GL_DEPTH_COMPONENT16: GLint = 0x81A5;
pub const GL_STENCIL_INDEX:     GLint = 0x1901;
pub const GL_STENCIL_INDEX8:    GLint = 0x8D48;

pub const GL_RENDERBUFFER_WIDTH:           GLint = 0x8D42;
pub const GL_RENDERBUFFER_HEIGHT:          GLint = 0x8D43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLint = 0x8D44;
pub const GL_RENDERBUFFER_RED_SIZE:        GLint = 0x8D50;
pub const GL_RENDERBUFFER_GREEN_SIZE:      GLint = 0x8D51;
pub const GL_RENDERBUFFER_BLUE_SIZE:       GLint = 0x8D52;
pub const GL_RENDERBUFFER_ALPHA_SIZE:      GLint = 0x8D53;
pub const GL_RENDERBUFFER_DEPTH_SIZE:      GLint = 0x8D54;
pub const GL_RENDERBUFFER_STENCIL_SIZE:    GLint = 0x8D55;

pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE:           GLint = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME:           GLint = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL:         GLint = 0x8CD2;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLint = 0x8CD3;

pub const GL_COLOR_ATTACHMENT0:  GLint = 0x8CE0;
pub const GL_DEPTH_ATTACHMENT:   GLint = 0x8D00;
pub const GL_STENCIL_ATTACHMENT: GLint = 0x8D20;

pub const GL_NONE: GLint = 0;

pub const GL_FRAMEBUFFER_COMPLETE:                      GLint = 0x8CD5;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT:         GLint = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLint = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS:         GLint = 0x8CD9;
pub const GL_FRAMEBUFFER_UNSUPPORTED:                   GLint = 0x8CDD;

pub const GL_FRAMEBUFFER_BINDING:   GLint = 0x8CA6;
pub const GL_RENDERBUFFER_BINDING:  GLint = 0x8CA7;
pub const GL_MAX_RENDERBUFFER_SIZE: GLint = 0x84E8;

pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLint = 0x0506;

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

mod ffi {
    use libc::c_int;

    use super::*;

    extern {
        pub fn glActiveTexture(texture: GLenum);

        pub fn glAttachShader(program: GLuint, shader: GLuint);

        pub fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);

        pub fn glBindBuffer(target: GLenum, buffer: GLuint);

        pub fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);

        pub fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);

        pub fn glBindTexture(target: GLenum, texture: GLuint);

        pub fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);

        pub fn glBlendEquation(mode: GLenum);

        pub fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);

        pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);

        pub fn glBlendFuncSeparate(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum,
                                   dstAlpha: GLenum);

        pub fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum);

        pub fn glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr,
                               data: *const GLvoid);

        pub fn glCheckFramebufferStatus(target: GLenum) -> GLenum;

        pub fn glClear(mask: GLbitfield);

        pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);

        pub fn glClearDepthf(depth: GLclampf);

        pub fn glClearStencil(s: GLint);

        pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

        pub fn glCompileShader(shader: GLuint);

        pub fn glCompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum,
                                      width: GLsizei, height: GLsizei, border: GLint,
                                      imageSize: GLsizei, data: *const GLvoid);

        pub fn glCompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint,
                                         yoffset: GLint, width: GLsizei, height: GLsizei,
                                         format: GLenum, imageSize: GLsizei, data: *const GLvoid);

        pub fn glCopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum,
                                x: GLint, y: GLint, width: GLsizei, height: GLsizei,
                                border: GLint);

        pub fn glCopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint,
                                   x: GLint, y: GLint, width: GLsizei, height: GLsizei);

        pub fn glCreateProgram() -> GLuint;

        pub fn glCreateShader(type_: GLenum) -> GLuint;

        pub fn glCullFace(mode: GLenum);

        pub fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint);

        pub fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);

        pub fn glDeleteProgram(program: GLuint);

        pub fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);

        pub fn glDeleteShader(shader: GLuint);

        pub fn glDeleteTextures(n: GLsizei, textures: *const GLuint);

        pub fn glDepthFunc(func: GLenum);

        pub fn glDepthMask(flag: GLboolean);

        pub fn glDepthRangef(zNear: GLclampf, zFar: GLclampf);

        pub fn glDetachShader(program: GLuint, shader: GLuint);

        pub fn glDisable(cap: GLenum);

        pub fn glDisableVertexAttribArray(index: GLuint);

        pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);

        pub fn glDrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid);

        pub fn glEnable(cap: GLenum);

        pub fn glEnableVertexAttribArray(index: GLuint);

        pub fn glFinish();

        pub fn glFlush();

        pub fn glFramebufferRenderbuffer(target: GLenum, attachment: GLenum,
                                         renderbuffertarget: GLenum, renderbuffer: GLuint);

        pub fn glFramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum,
                                      texture: GLuint, level: GLint);

        pub fn glFrontFace(mode: GLenum);

        pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);

        pub fn glGenerateMipmap(target: GLenum);

        pub fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);

        pub fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);

        pub fn glGenTextures(n: GLsizei, textures: *mut GLuint);

        pub fn glGetActiveAttrib(program: GLuint, index: GLuint, bufsize: GLsizei,
                                 length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum,
                                 name: *mut GLchar);

        pub fn glGetActiveUniform(program: GLuint, index: GLuint, bufsize: GLsizei,
                                  length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum,
                                  name: *mut GLchar);

        pub fn glGetAttachedShaders(program: GLuint, maxcount: GLsizei, count: *mut GLsizei,
                                    shaders: *mut GLuint);

        pub fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> c_int;

        pub fn glGetBooleanv(pname: GLenum, params: *mut GLboolean);

        pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);

        pub fn glGetError() -> GLenum;

        pub fn glGetFloatv(pname: GLenum, params: *mut GLfloat);

        pub fn glGetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum,
                                                     pname: GLenum, params: *mut GLint);

        pub fn glGetIntegerv(pname: GLenum, params: *mut GLint);

        pub fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);

        pub fn glGetProgramInfoLog(program: GLuint, bufsize: GLsizei, length: *mut GLsizei,
                                   infolog: *mut GLchar);

        pub fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);

        pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);

        pub fn glGetShaderInfoLog(shader: GLuint, bufsize: GLsizei, length: *mut GLsizei,
                                  infolog: *mut GLchar);

        pub fn glGetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum,
                                          range: *mut GLint, precision: *mut GLint);

        pub fn glGetShaderSource(shader: GLuint, bufsize: GLsizei, length: *mut GLsizei,
                                 source: *mut GLchar);

        pub fn glGetString(name: GLenum) -> *const GLubyte;

        pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);

        pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);

        pub fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);

        pub fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint);

        pub fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> c_int;

        pub fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat);

        pub fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint);

        pub fn glGetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut GLvoid);

        pub fn glHint(target: GLenum, mode: GLenum);

        pub fn glIsBuffer(buffer: GLuint) -> GLboolean;

        pub fn glIsEnabled(cap: GLenum) -> GLboolean;

        pub fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean;

        pub fn glIsProgram(program: GLuint) -> GLboolean;

        pub fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean;

        pub fn glIsShader(shader: GLuint) -> GLboolean;

        pub fn glIsTexture(texture: GLuint) -> GLboolean;

        pub fn glLineWidth(width: GLfloat);

        pub fn glLinkProgram(program: GLuint);

        pub fn glPixelStorei(pname: GLenum, param: GLint);

        pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);

        pub fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum,
                            type_: GLenum, pixels: *mut GLvoid);

        pub fn glReleaseShaderCompiler();

        pub fn glRenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei,
                                     height: GLsizei);

        pub fn glSampleCoverage(value: GLclampf, invert: GLboolean);

        pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

        pub fn glShaderBinary(n: GLsizei, shaders: *const GLuint, binaryformat: GLenum,
                              binary: *const GLvoid, length: GLsizei);

        pub fn glShaderSource(shader: GLuint, count: GLsizei, string: *mut *const GLchar,
                              length: *const GLint);

        pub fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint);

        pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);

        pub fn glStencilMask(mask: GLuint);

        pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint);

        pub fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);

        pub fn glStencilOpSeparate(face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum);

        pub fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei,
                            height: GLsizei, border: GLint, format: GLenum, type_: GLenum,
                            pixels: *const GLvoid);

        pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);

        pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);

        pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);

        pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);

        pub fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint,
                               width: GLsizei, height: GLsizei, format: GLenum,
                               type_: GLenum, pixels: *const GLvoid);

        pub fn glUniform1f(location: GLint, x: GLfloat);

        pub fn glUniform1fv(location: GLint, count: GLsizei, v: *const GLfloat);

        pub fn glUniform1i(location: GLint, x: GLint);

        pub fn glUniform1iv(location: GLint, count: GLsizei, v: *const GLint);

        pub fn glUniform2f(location: GLint, x: GLfloat, y: GLfloat);

        pub fn glUniform2fv(location: GLint, count: GLsizei, v: *const GLfloat);

        pub fn glUniform2i(location: GLint, x: GLint, y: GLint);

        pub fn glUniform2iv(location: GLint, count: GLsizei, v: *const GLint);

        pub fn glUniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat);

        pub fn glUniform3fv(location: GLint, count: GLsizei, v: *const GLfloat);

        pub fn glUniform3i(location: GLint, x: GLint, y: GLint, z: GLint);

        pub fn glUniform3iv(location: GLint, count: GLsizei, v: *const GLint);

        pub fn glUniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

        pub fn glUniform4fv(location: GLint, count: GLsizei, v: *const GLfloat);

        pub fn glUniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint);

        pub fn glUniform4iv(location: GLint, count: GLsizei, v: *const GLint);

        pub fn glUniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean,
                                  value: *const GLfloat);

        pub fn glUniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean,
                                  value: *const GLfloat);

        pub fn glUniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean,
                                  value: *const GLfloat);

        pub fn glUseProgram(program: GLuint);

        pub fn glValidateProgram(program: GLuint);

        pub fn glVertexAttrib1f(indx: GLuint, x: GLfloat);

        pub fn glVertexAttrib1fv(indx: GLuint, values: *const GLfloat);

        pub fn glVertexAttrib2f(indx: GLuint, x: GLfloat, y: GLfloat);

        pub fn glVertexAttrib2fv(indx: GLuint, values: *const GLfloat);

        pub fn glVertexAttrib3f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

        pub fn glVertexAttrib3fv(indx: GLuint, values: *const GLfloat);

        pub fn glVertexAttrib4f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

        pub fn glVertexAttrib4fv(indx: GLuint, values: *const GLfloat);

        pub fn glVertexAttribPointer(indx: GLuint, size: GLint, type_: GLenum,
                                     normalized: GLboolean, stride: GLsizei, ptr: *const GLvoid);

        pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    }
}