// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0(the "License");                                |
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
// | Author: Sean Kerr <sean@metatomic.io>                                                         |
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

use std::ffi::CStr;
use std::ffi::CString;
use std::mem::size_of;
use std::str::from_utf8;

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
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct Active {
    pub name:   String,
    pub size:   GLint,
    pub type_:  GLenum
}

pub struct ShaderPrecisionFormat {
    pub precision: GLint,
    pub range:     [GLint; 2]
}

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

// OpenGL ES core versions
pub const GL_ES_VERSION_2_0: GLint = 1;

// ClearBufferMask
pub const GL_DEPTH_BUFFER_BIT:   GLuint = 0x00000100;
pub const GL_STENCIL_BUFFER_BIT: GLuint = 0x00000400;
pub const GL_COLOR_BUFFER_BIT:   GLuint = 0x00004000;

// boolean
pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE:  GLboolean = 1;

// BeginMode
pub const GL_POINTS:         GLuint = 0x0000;
pub const GL_LINES:          GLuint = 0x0001;
pub const GL_LINE_LOOP:      GLuint = 0x0002;
pub const GL_LINE_STRIP:     GLuint = 0x0003;
pub const GL_TRIANGLES:      GLuint = 0x0004;
pub const GL_TRIANGLE_STRIP: GLuint = 0x0005;
pub const GL_TRIANGLE_FAN:   GLuint = 0x0006;

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
pub const GL_ZERO:                GLuint = 0;
pub const GL_ONE:                 GLuint = 1;
pub const GL_SRC_COLOR:           GLuint = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLuint = 0x0301;
pub const GL_SRC_ALPHA:           GLuint = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLuint = 0x0303;
pub const GL_DST_ALPHA:           GLuint = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLuint = 0x0305;

// BlendingFactorSrc
//      GL_ZERO
//      GL_ONE
pub const GL_DST_COLOR:           GLuint = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLuint = 0x0307;
pub const GL_SRC_ALPHA_SATURATE:  GLuint = 0x0308;
//      GL_SRC_ALPHA
//      GL_ONE_MINUS_SRC_ALPHA
//      GL_DST_ALPHA
//      GL_ONE_MINUS_DST_ALPHA

// BlendEquationSeparate
pub const GL_FUNC_ADD:             GLuint = 0x8006;
pub const GL_BLEND_EQUATION:       GLuint = 0x8009;
pub const GL_BLEND_EQUATION_RGB:   GLuint = 0x8009; // same as BLEND_EQUATION
pub const GL_BLEND_EQUATION_ALPHA: GLuint = 0x883D;

// BlendSubtract
pub const GL_FUNC_SUBTRACT:         GLuint = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: GLuint = 0x800B;

// Separate Blend Functions
pub const GL_BLEND_DST_RGB:            GLuint = 0x80C8;
pub const GL_BLEND_SRC_RGB:            GLuint = 0x80C9;
pub const GL_BLEND_DST_ALPHA:          GLuint = 0x80CA;
pub const GL_BLEND_SRC_ALPHA:          GLuint = 0x80CB;
pub const GL_CONSTANT_COLOR:           GLuint = 0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLuint = 0x8002;
pub const GL_CONSTANT_ALPHA:           GLuint = 0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLuint = 0x8004;
pub const GL_BLEND_COLOR:              GLuint = 0x8005;

// Buffer Objects
pub const GL_ARRAY_BUFFER:                 GLuint = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER:         GLuint = 0x8893;
pub const GL_ARRAY_BUFFER_BINDING:         GLuint = 0x8894;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLuint = 0x8895;

pub const GL_STREAM_DRAW:  GLuint = 0x88E0;
pub const GL_STATIC_DRAW:  GLuint = 0x88E4;
pub const GL_DYNAMIC_DRAW: GLuint = 0x88E8;

pub const GL_BUFFER_SIZE:  GLuint = 0x8764;
pub const GL_BUFFER_USAGE: GLuint = 0x8765;

pub const GL_CURRENT_VERTEX_ATTRIB: GLuint = 0x8626;

// CullFaceMode
pub const GL_FRONT:          GLuint = 0x0404;
pub const GL_BACK:           GLuint = 0x0405;
pub const GL_FRONT_AND_BACK: GLuint = 0x0408;

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
pub const GL_TEXTURE_2D:               GLuint = 0x0DE1;
pub const GL_CULL_FACE:                GLuint = 0x0B44;
pub const GL_BLEND:                    GLuint = 0x0BE2;
pub const GL_DITHER:                   GLuint = 0x0BD0;
pub const GL_STENCIL_TEST:             GLuint = 0x0B90;
pub const GL_DEPTH_TEST:               GLuint = 0x0B71;
pub const GL_SCISSOR_TEST:             GLuint = 0x0C11;
pub const GL_POLYGON_OFFSET_FILL:      GLuint = 0x8037;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLuint = 0x809E;
pub const GL_SAMPLE_COVERAGE:          GLuint = 0x80A0;

// ErrorCode
pub const GL_NO_ERROR:          GLuint = 0;
pub const GL_INVALID_ENUM:      GLuint = 0x0500;
pub const GL_INVALID_VALUE:     GLuint = 0x0501;
pub const GL_INVALID_OPERATION: GLuint = 0x0502;
pub const GL_OUT_OF_MEMORY:     GLuint = 0x0505;

// FrontFaceDirection
pub const GL_CW:  GLint = 0x0900;
pub const GL_CCW: GLint = 0x0901;

// GetPName
pub const GL_LINE_WIDTH:                   GLuint = 0x0B21;
pub const GL_ALIASED_POINT_SIZE_RANGE:     GLuint = 0x846D;
pub const GL_ALIASED_LINE_WIDTH_RANGE:     GLuint = 0x846E;
pub const GL_CULL_FACE_MODE:               GLuint = 0x0B45;
pub const GL_FRONT_FACE:                   GLuint = 0x0B46;
pub const GL_DEPTH_RANGE:                  GLuint = 0x0B70;
pub const GL_DEPTH_WRITEMASK:              GLuint = 0x0B72;
pub const GL_DEPTH_CLEAR_VALUE:            GLuint = 0x0B73;
pub const GL_DEPTH_FUNC:                   GLuint = 0x0B74;
pub const GL_STENCIL_CLEAR_VALUE:          GLuint = 0x0B91;
pub const GL_STENCIL_FUNC:                 GLuint = 0x0B92;
pub const GL_STENCIL_FAIL:                 GLuint = 0x0B94;
pub const GL_STENCIL_PASS_DEPTH_FAIL:      GLuint = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS:      GLuint = 0x0B96;
pub const GL_STENCIL_REF:                  GLuint = 0x0B97;
pub const GL_STENCIL_VALUE_MASK:           GLuint = 0x0B93;
pub const GL_STENCIL_WRITEMASK:            GLuint = 0x0B98;
pub const GL_STENCIL_BACK_FUNC:            GLuint = 0x8800;
pub const GL_STENCIL_BACK_FAIL:            GLuint = 0x8801;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLuint = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLuint = 0x8803;
pub const GL_STENCIL_BACK_REF:             GLuint = 0x8CA3;
pub const GL_STENCIL_BACK_VALUE_MASK:      GLuint = 0x8CA4;
pub const GL_STENCIL_BACK_WRITEMASK:       GLuint = 0x8CA5;
pub const GL_VIEWPORT:                     GLuint = 0x0BA2;
pub const GL_SCISSOR_BOX:                  GLuint = 0x0C10;
//      GL_SCISSOR_TEST
pub const GL_COLOR_CLEAR_VALUE:            GLuint = 0x0C22;
pub const GL_COLOR_WRITEMASK:              GLuint = 0x0C23;
pub const GL_UNPACK_ALIGNMENT:             GLuint = 0x0CF5;
pub const GL_PACK_ALIGNMENT:               GLuint = 0x0D05;
pub const GL_MAX_TEXTURE_SIZE:             GLuint = 0x0D33;
pub const GL_MAX_VIEWPORT_DIMS:            GLuint = 0x0D3A;
pub const GL_SUBPIXEL_BITS:                GLuint = 0x0D50;
pub const GL_RED_BITS:                     GLuint = 0x0D52;
pub const GL_GREEN_BITS:                   GLuint = 0x0D53;
pub const GL_BLUE_BITS:                    GLuint = 0x0D54;
pub const GL_ALPHA_BITS:                   GLuint = 0x0D55;
pub const GL_DEPTH_BITS:                   GLuint = 0x0D56;
pub const GL_STENCIL_BITS:                 GLuint = 0x0D57;
pub const GL_POLYGON_OFFSET_UNITS:         GLuint = 0x2A00;
//      GL_POLYGON_OFFSET_FILL
pub const GL_POLYGON_OFFSET_FACTOR:        GLuint = 0x8038;
pub const GL_TEXTURE_BINDING_2D:           GLuint = 0x8069;
pub const GL_SAMPLE_BUFFERS:               GLuint = 0x80A8;
pub const GL_SAMPLES:                      GLuint = 0x80A9;
pub const GL_SAMPLE_COVERAGE_VALUE:        GLuint = 0x80AA;
pub const GL_SAMPLE_COVERAGE_INVERT:       GLuint = 0x80AB;

// GetTextureParameter
//      GL_TEXTURE_MAG_FILTER
//      GL_TEXTURE_MIN_FILTER
//      GL_TEXTURE_WRAP_S
//      GL_TEXTURE_WRAP_T

pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLuint = 0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS:     GLuint = 0x86A3;

// HintMode
pub const GL_DONT_CARE: GLuint = 0x1100;
pub const GL_FASTEST:   GLuint = 0x1101;
pub const GL_NICEST:    GLuint = 0x1102;

// HintTarget
pub const GL_GENERATE_MIPMAP_HINT: GLint = 0x8192;

// DataType
pub const GL_BYTE:           GLuint = 0x1400;
pub const GL_UNSIGNED_BYTE:  GLuint = 0x1401;
pub const GL_SHORT:          GLuint = 0x1402;
pub const GL_UNSIGNED_SHORT: GLuint = 0x1403;
pub const GL_INT:            GLuint = 0x1404;
pub const GL_UNSIGNED_INT:   GLuint = 0x1405;
pub const GL_FLOAT:          GLuint = 0x1406;
pub const GL_FIXED:          GLuint = 0x140C;

// PixelFormat
pub const GL_DEPTH_COMPONENT: GLuint = 0x1902;
pub const GL_ALPHA:           GLuint = 0x1906;
pub const GL_RGB:             GLuint = 0x1907;
pub const GL_RGBA:            GLuint = 0x1908;
pub const GL_LUMINANCE:       GLuint = 0x1909;
pub const GL_LUMINANCE_ALPHA: GLuint = 0x190A;

// PixelType
//      GL_UNSIGNED_BYTE
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLuint = 0x8033;
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLuint = 0x8034;
pub const GL_UNSIGNED_SHORT_5_6_5:   GLuint = 0x8363;

// Shaders
pub const GL_FRAGMENT_SHADER:                  GLuint = 0x8B30;
pub const GL_VERTEX_SHADER:                    GLuint = 0x8B31;
pub const GL_MAX_VERTEX_ATTRIBS:               GLuint = 0x8869;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS:       GLuint = 0x8DFB;
pub const GL_MAX_VARYING_VECTORS:              GLuint = 0x8DFC;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLuint = 0x8B4D;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS:   GLuint = 0x8B4C;
pub const GL_MAX_TEXTURE_IMAGE_UNITS:          GLuint = 0x8872;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS:     GLuint = 0x8DFD;
pub const GL_SHADER_TYPE:                      GLuint = 0x8B4F;
pub const GL_DELETE_STATUS:                    GLuint = 0x8B80;
pub const GL_LINK_STATUS:                      GLuint = 0x8B82;
pub const GL_VALIDATE_STATUS:                  GLuint = 0x8B83;
pub const GL_ATTACHED_SHADERS:                 GLuint = 0x8B85;
pub const GL_ACTIVE_UNIFORMS:                  GLuint = 0x8B86;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH:        GLuint = 0x8B87;
pub const GL_ACTIVE_ATTRIBUTES:                GLuint = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH:      GLuint = 0x8B8A;
pub const GL_SHADING_LANGUAGE_VERSION:         GLuint = 0x8B8C;
pub const GL_CURRENT_PROGRAM:                  GLuint = 0x8B8D;

// StencilFunction
pub const GL_NEVER:    GLuint = 0x0200;
pub const GL_LESS:     GLuint = 0x0201;
pub const GL_EQUAL:    GLuint = 0x0202;
pub const GL_LEQUAL:   GLuint = 0x0203;
pub const GL_GREATER:  GLuint = 0x0204;
pub const GL_NOTEQUAL: GLuint = 0x0205;
pub const GL_GEQUAL:   GLuint = 0x0206;
pub const GL_ALWAYS:   GLuint = 0x0207;

// StencilOp
//      GL_ZERO
pub const GL_KEEP:      GLuint = 0x1E00;
pub const GL_REPLACE:   GLuint = 0x1E01;
pub const GL_INCR:      GLuint = 0x1E02;
pub const GL_DECR:      GLuint = 0x1E03;
pub const GL_INVERT:    GLuint = 0x150A;
pub const GL_INCR_WRAP: GLuint = 0x8507;
pub const GL_DECR_WRAP: GLuint = 0x8508;

// StringName
pub const GL_VENDOR:     GLuint = 0x1F00;
pub const GL_RENDERER:   GLuint = 0x1F01;
pub const GL_VERSION:    GLuint = 0x1F02;
pub const GL_EXTENSIONS: GLuint = 0x1F03;

// TextureMagFilter
pub const GL_NEAREST: GLuint = 0x2600;
pub const GL_LINEAR:  GLuint = 0x2601;

// TextureMinFilter
//      GL_NEAREST
//      GL_LINEAR
pub const GL_NEAREST_MIPMAP_NEAREST: GLuint = 0x2700;
pub const GL_LINEAR_MIPMAP_NEAREST:  GLuint = 0x2701;
pub const GL_NEAREST_MIPMAP_LINEAR:  GLuint = 0x2702;
pub const GL_LINEAR_MIPMAP_LINEAR:   GLuint = 0x2703;

// TextureParameterName
pub const GL_TEXTURE_MAG_FILTER: GLuint = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: GLuint = 0x2801;
pub const GL_TEXTURE_WRAP_S:     GLuint = 0x2802;
pub const GL_TEXTURE_WRAP_T:     GLuint = 0x2803;

// TextureTarget
//      GL_TEXTURE_2D
pub const GL_TEXTURE:                     GLuint = 0x1702;
pub const GL_TEXTURE_CUBE_MAP:            GLuint = 0x8513;
pub const GL_TEXTURE_BINDING_CUBE_MAP:    GLuint = 0x8514;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLuint = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLuint = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLuint = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLuint = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLuint = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLuint = 0x851A;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE:   GLuint = 0x851C;

// TextureUnit
pub const GL_TEXTURE0:       GLuint = 0x84C0;
pub const GL_TEXTURE1:       GLuint = 0x84C1;
pub const GL_TEXTURE2:       GLuint = 0x84C2;
pub const GL_TEXTURE3:       GLuint = 0x84C3;
pub const GL_TEXTURE4:       GLuint = 0x84C4;
pub const GL_TEXTURE5:       GLuint = 0x84C5;
pub const GL_TEXTURE6:       GLuint = 0x84C6;
pub const GL_TEXTURE7:       GLuint = 0x84C7;
pub const GL_TEXTURE8:       GLuint = 0x84C8;
pub const GL_TEXTURE9:       GLuint = 0x84C9;
pub const GL_TEXTURE10:      GLuint = 0x84CA;
pub const GL_TEXTURE11:      GLuint = 0x84CB;
pub const GL_TEXTURE12:      GLuint = 0x84CC;
pub const GL_TEXTURE13:      GLuint = 0x84CD;
pub const GL_TEXTURE14:      GLuint = 0x84CE;
pub const GL_TEXTURE15:      GLuint = 0x84CF;
pub const GL_TEXTURE16:      GLuint = 0x84D0;
pub const GL_TEXTURE17:      GLuint = 0x84D1;
pub const GL_TEXTURE18:      GLuint = 0x84D2;
pub const GL_TEXTURE19:      GLuint = 0x84D3;
pub const GL_TEXTURE20:      GLuint = 0x84D4;
pub const GL_TEXTURE21:      GLuint = 0x84D5;
pub const GL_TEXTURE22:      GLuint = 0x84D6;
pub const GL_TEXTURE23:      GLuint = 0x84D7;
pub const GL_TEXTURE24:      GLuint = 0x84D8;
pub const GL_TEXTURE25:      GLuint = 0x84D9;
pub const GL_TEXTURE26:      GLuint = 0x84DA;
pub const GL_TEXTURE27:      GLuint = 0x84DB;
pub const GL_TEXTURE28:      GLuint = 0x84DC;
pub const GL_TEXTURE29:      GLuint = 0x84DD;
pub const GL_TEXTURE30:      GLuint = 0x84DE;
pub const GL_TEXTURE31:      GLuint = 0x84DF;
pub const GL_ACTIVE_TEXTURE: GLuint = 0x84E0;

// TextureWrapMode
pub const GL_REPEAT:          GLuint = 0x2901;
pub const GL_CLAMP_TO_EDGE:   GLuint = 0x812F;
pub const GL_MIRRORED_REPEAT: GLuint = 0x8370;

// Uniform Types
pub const GL_FLOAT_VEC2:   GLuint = 0x8B50;
pub const GL_FLOAT_VEC3:   GLuint = 0x8B51;
pub const GL_FLOAT_VEC4:   GLuint = 0x8B52;
pub const GL_INT_VEC2:     GLuint = 0x8B53;
pub const GL_INT_VEC3:     GLuint = 0x8B54;
pub const GL_INT_VEC4:     GLuint = 0x8B55;
pub const GL_BOOL:         GLuint = 0x8B56;
pub const GL_BOOL_VEC2:    GLuint = 0x8B57;
pub const GL_BOOL_VEC3:    GLuint = 0x8B58;
pub const GL_BOOL_VEC4:    GLuint = 0x8B59;
pub const GL_FLOAT_MAT2:   GLuint = 0x8B5A;
pub const GL_FLOAT_MAT3:   GLuint = 0x8B5B;
pub const GL_FLOAT_MAT4:   GLuint = 0x8B5C;
pub const GL_SAMPLER_2D:   GLuint = 0x8B5E;
pub const GL_SAMPLER_CUBE: GLuint = 0x8B60;

// Vertex Arrays
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED:        GLuint = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE:           GLuint = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE:         GLuint = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE:           GLuint = 0x8625;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED:     GLuint = 0x886A;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER:        GLuint = 0x8645;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLuint = 0x889F;

// Read Format
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE:   GLuint = 0x8B9A;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLuint = 0x8B9B;

// Shader Source
pub const GL_COMPILE_STATUS:       GLuint = 0x8B81;
pub const GL_INFO_LOG_LENGTH:      GLuint = 0x8B84;
pub const GL_SHADER_SOURCE_LENGTH: GLuint = 0x8B88;
pub const GL_SHADER_COMPILER:      GLuint = 0x8DFA;

// Shader Binary
pub const GL_SHADER_BINARY_FORMATS:     GLuint = 0x8DF8;
pub const GL_NUM_SHADER_BINARY_FORMATS: GLuint = 0x8DF9;

// Shader Precision-Specified Types
pub const GL_LOW_FLOAT:    GLuint = 0x8DF0;
pub const GL_MEDIUM_FLOAT: GLuint = 0x8DF1;
pub const GL_HIGH_FLOAT:   GLuint = 0x8DF2;
pub const GL_LOW_INT:      GLuint = 0x8DF3;
pub const GL_MEDIUM_INT:   GLuint = 0x8DF4;
pub const GL_HIGH_INT:     GLuint = 0x8DF5;

// Framebuffer Object.
pub const GL_FRAMEBUFFER:  GLuint = 0x8D40;
pub const GL_RENDERBUFFER: GLuint = 0x8D41;

pub const GL_RGBA4:             GLuint = 0x8056;
pub const GL_RGB5_A1:           GLuint = 0x8057;
pub const GL_RGB565:            GLuint = 0x8D62;
pub const GL_DEPTH_COMPONENT16: GLuint = 0x81A5;
pub const GL_STENCIL_INDEX:     GLuint = 0x1901;
pub const GL_STENCIL_INDEX8:    GLuint = 0x8D48;

pub const GL_RENDERBUFFER_WIDTH:           GLuint = 0x8D42;
pub const GL_RENDERBUFFER_HEIGHT:          GLuint = 0x8D43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLuint = 0x8D44;
pub const GL_RENDERBUFFER_RED_SIZE:        GLuint = 0x8D50;
pub const GL_RENDERBUFFER_GREEN_SIZE:      GLuint = 0x8D51;
pub const GL_RENDERBUFFER_BLUE_SIZE:       GLuint = 0x8D52;
pub const GL_RENDERBUFFER_ALPHA_SIZE:      GLuint = 0x8D53;
pub const GL_RENDERBUFFER_DEPTH_SIZE:      GLuint = 0x8D54;
pub const GL_RENDERBUFFER_STENCIL_SIZE:    GLuint = 0x8D55;

pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE:           GLuint = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME:           GLuint = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL:         GLuint = 0x8CD2;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLuint = 0x8CD3;

pub const GL_COLOR_ATTACHMENT0:  GLuint = 0x8CE0;
pub const GL_DEPTH_ATTACHMENT:   GLuint = 0x8D00;
pub const GL_STENCIL_ATTACHMENT: GLuint = 0x8D20;

pub const GL_NONE: GLuint = 0;

pub const GL_FRAMEBUFFER_COMPLETE:                      GLuint = 0x8CD5;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT:         GLuint = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLuint = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS:         GLuint = 0x8CD9;
pub const GL_FRAMEBUFFER_UNSUPPORTED:                   GLuint = 0x8CDD;

pub const GL_FRAMEBUFFER_BINDING:   GLuint = 0x8CA6;
pub const GL_RENDERBUFFER_BINDING:  GLuint = 0x8CA7;
pub const GL_MAX_RENDERBUFFER_SIZE: GLuint = 0x84E8;

pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLuint = 0x0506;

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

pub fn active_texture(texture: GLenum) {
    unsafe {
        ffi::glActiveTexture(texture)
    }
}

pub fn attach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        ffi::glAttachShader(program, shader)
    }
}

pub fn bind_attrib_location(program: GLuint, index: GLuint, name: &str) {
    unsafe {
        let c_str = CString::new(name).unwrap();

        ffi::glBindAttribLocation(program, index, c_str.as_ptr() as *const c_char)
    }
}

pub fn bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe {
        ffi::glBindBuffer(target, buffer)
    }
}

pub fn bind_framebuffer(target: GLenum, framebuffer: GLuint) {
    unsafe {
        ffi::glBindFramebuffer(target, framebuffer)
    }
}

pub fn bind_renderbuffer(target: GLenum, renderbuffer: GLuint) {
    unsafe {
        ffi::glBindRenderbuffer(target, renderbuffer)
    }
}

pub fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe {
        ffi::glBindTexture(target, texture)
    }
}

pub fn blend_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    unsafe {
        ffi::glBlendColor(red, green, blue, alpha)
    }
}

pub fn blend_equation(mode: GLenum) {
    unsafe {
        ffi::glBlendEquation(mode)
    }
}

pub fn blend_equation_separate(mode_rgb: GLenum, mode_alpha: GLenum) {
    unsafe {
        ffi::glBlendEquationSeparate(mode_rgb, mode_alpha)
    }
}

pub fn blend_func(src_factor: GLenum, dst_factor: GLenum) {
    unsafe {
        ffi::glBlendFunc(src_factor, dst_factor)
    }
}

pub fn blend_func_separate(src_rgb: GLenum, dst_rgb: GLenum, src_alpha: GLenum, dst_alpha: GLenum) {
    unsafe {
        ffi::glBlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha)
    }
}

pub fn buffer_data<T>(target: GLenum, buffer: &[T], usage: GLenum) {
    unsafe {
    	let ptr = if buffer.len() == 0 { 0 as *const GLvoid }
    				else { buffer.as_ptr() as *const GLvoid };

        ffi::glBufferData(target, (buffer.len() * size_of::<T>()) as GLsizeiptr, ptr, usage)
    }
}

pub fn buffer_sub_data<T>(target: GLenum, offset: GLintptr, buffer: &[T]) {
    unsafe {
        let size = size_of::<T>();
        let ptr = if buffer.len() == 0 { 0 as *const GLvoid }
        			else { buffer.as_ptr() as *const GLvoid };

        ffi::glBufferSubData(target, offset * (size as GLintptr),
                             (buffer.len() * size) as GLsizeiptr, ptr)
    }
}

pub fn check_framebuffer_status(target: GLenum) -> GLenum {
    unsafe {
        ffi::glCheckFramebufferStatus(target)
    }
}

pub fn clear(mask: GLbitfield) {
    unsafe {
        ffi::glClear(mask)
    }
}

pub fn clear_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    unsafe {
        ffi::glClearColor(red, green, blue, alpha)
    }
}

pub fn clear_depthf(depth: GLclampf) {
    unsafe {
        ffi::glClearDepthf(depth)
    }
}

pub fn clear_stencil(stencil: GLint) {
    unsafe {
        ffi::glClearStencil(stencil)
    }
}

pub fn color_mask(red: bool, green: bool, blue: bool, alpha: bool) {
    unsafe {
        ffi::glColorMask(red as GLboolean,
                         green as GLboolean,
                         blue as GLboolean,
                         alpha as GLboolean)
    }
}

pub fn compile_shader(shader: GLuint) {
    unsafe {
        ffi::glCompileShader(shader)
    }
}

pub fn compressed_tex_image_2d<T>(target: GLenum, level: GLint, internal_format: GLenum,
                                  width: GLsizei, height: GLsizei, border: GLint,
                                  image_size: GLsizei, buffer: &[T]) {
    unsafe {
    	let ptr = if buffer.len() == 0 { 0 as *const GLvoid }
    				else { buffer.as_ptr() as *const GLvoid };

        ffi::glCompressedTexImage2D(target, level, internal_format, width, height, border,
                                    image_size, ptr)
    }
}

pub fn compressed_tex_sub_image_2d<T>(target: GLenum, level: GLint, x_offset: GLint,
                                      y_offset: GLint, width: GLsizei, height: GLsizei,
                                      format: GLenum, image_size: GLsizei, buffer: &[T]) {
    unsafe {
		let ptr = if buffer.len() == 0 { 0 as *const GLvoid }
					else { buffer.as_ptr() as *const GLvoid };

        ffi::glCompressedTexSubImage2D(target, level, x_offset, y_offset, width, height, format,
                                       image_size, ptr)
    }
}

pub fn copy_tex_image_2d(target: GLenum, level: GLint, internal_format: GLenum, x: GLint, y: GLint,
                         width: GLsizei, height: GLsizei, border: GLint) {
    unsafe {
        ffi::glCopyTexImage2D(target, level, internal_format, x, y, width, height, border)
    }
}

pub fn copy_tex_sub_image_2d(target: GLenum, level: GLint, x_offset: GLint, y_offset: GLint,
                             x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        ffi::glCopyTexSubImage2D(target, level, x_offset, y_offset, x,  y, width, height)
    }
}

pub fn create_program() -> GLuint {
    unsafe {
        ffi::glCreateProgram()
    }
}

pub fn create_shader(type_: GLenum) -> GLuint {
    unsafe {
        ffi::glCreateShader(type_)
    }
}

pub fn cull_face(mode: GLenum) {
    unsafe {
        ffi::glCullFace(mode)
    }
}

pub fn delete_buffers(buffers: &[GLuint]) {
    unsafe {
        ffi::glDeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr())
    }
}

pub fn delete_framebuffers(framebuffers: &[GLuint]) {
    unsafe {
        ffi::glDeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
    }
}

pub fn delete_program(program: GLuint) {
    unsafe {
        ffi::glDeleteProgram(program)
    }
}

pub fn delete_renderbuffers(renderbuffers: &[GLuint]) {
    unsafe {
        ffi::glDeleteRenderbuffers(renderbuffers.len() as GLsizei, renderbuffers.as_ptr())
    }
}

pub fn delete_shader(shader: GLuint) {
    unsafe {
        ffi::glDeleteShader(shader)
    }
}

pub fn delete_textures(textures: &[GLuint]) {
    unsafe {
        ffi::glDeleteTextures(textures.len() as GLsizei, textures.as_ptr())
    }
}

pub fn depth_func(func: GLenum) {
    unsafe {
        ffi::glDepthFunc(func)
    }
}

pub fn depth_mask(flag: bool) {
    unsafe {
        ffi::glDepthMask(flag as GLboolean)
    }
}

pub fn depth_rangef(z_near: GLclampf, z_far: GLclampf) {
    unsafe {
        ffi::glDepthRangef(z_near, z_far)
    }
}

pub fn detach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        ffi::glDetachShader(program, shader)
    }
}

pub fn disable(feature: GLenum) {
    unsafe {
        ffi::glDisable(feature)
    }
}

pub fn disable_vertex_attrib_array(index: GLuint) {
    unsafe {
        ffi::glDisableVertexAttribArray(index)
    }
}

pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe {
        ffi::glDrawArrays(mode, first, count)
    }
}

pub fn draw_elements<T>(mode: GLenum, count: GLsizei, type_: GLenum, indices: &[T]) {
    unsafe {
		let ptr = if indices.len() == 0 { 0 as *const GLvoid }
					else { indices.as_ptr() as *const GLvoid };

        ffi::glDrawElements(mode, count, type_, ptr)
    }
}

pub fn enable(feature: GLenum) {
    unsafe {
        ffi::glEnable(feature)
    }
}

pub fn enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        ffi::glEnableVertexAttribArray(index)
    }
}

pub fn finish() {
    unsafe {
        ffi::glFinish()
    }
}

pub fn flush() {
    unsafe {
        ffi::glFlush()
    }
}

pub fn framebuffer_renderbuffer(target: GLenum, attachment: GLenum,
                                renderbuffer_target: GLenum, renderbuffer: GLuint) {
    unsafe {
        ffi::glFramebufferRenderbuffer(target, attachment, renderbuffer_target, renderbuffer)
    }
}

pub fn framebuffer_texture_2d(target: GLenum, attachment: GLenum, texture_target: GLenum,
                              texture: GLuint, level: GLint) {
    unsafe {
        ffi::glFramebufferTexture2D(target, attachment, texture_target, texture, level)
    }
}

pub fn front_face(mode: GLenum) {
    unsafe {
        ffi::glFrontFace(mode)
    }
}

pub fn gen_buffers(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenBuffers(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn generate_mipmap(target: GLenum) {
    unsafe {
        ffi::glGenerateMipmap(target)
    }
}

pub fn gen_framebuffers(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenFramebuffers(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn gen_renderbuffers(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenRenderbuffers(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn gen_textures(count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut vec: Vec<GLuint> = Vec::with_capacity(count as usize);

        ffi::glGenTextures(count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec
    }
}

pub fn get_active_attrib(program: GLuint, index: GLuint) -> Option<Active> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut size:   GLint = 0;
        let mut type_:  GLenum = 0;

        let mut name = String::with_capacity(256);

        ffi::glGetActiveAttrib(program, index, 256, &mut length, &mut size, &mut type_,
                               name.as_mut_vec().as_mut_ptr() as *mut GLchar);

        if length > 0 {
            name.as_mut_vec().set_len(length as usize);
            name.truncate(length as usize);

            Some(Active{ name:  name,
                         size:  size,
                         type_: type_ })
        } else {
            None
        }
    }
}

pub fn get_active_uniform(program: GLuint, index: GLuint) -> Option<Active> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut size:   GLint = 0;
        let mut type_:  GLenum = 0;

        let mut name = String::with_capacity(256);

        ffi::glGetActiveUniform(program, index, 256, &mut length, &mut size, &mut type_,
                                name.as_mut_vec().as_mut_ptr() as *mut GLchar);

        if length > 0 {
            name.as_mut_vec().set_len(length as usize);
            name.truncate(length as usize);

            Some(Active{ name:  name,
                         size:  size,
                         type_: type_ })
        } else {
            None
        }
    }
}

pub fn get_attached_shaders(program: GLuint, max_count: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut count: GLsizei = 0;
        let mut vec:   Vec<GLuint> = Vec::with_capacity(max_count as usize);

        ffi::glGetAttachedShaders(program, max_count, &mut count, vec.as_mut_ptr());

        vec.set_len(count as usize);
        vec.truncate(count as usize);
        vec
    }
}

pub fn get_attrib_location(program: GLuint, name: &str) -> GLint {
    unsafe {
        let c_str = CString::new(name).unwrap();

        ffi::glGetAttribLocation(program, c_str.as_ptr() as *const c_char)
    }
}

pub fn get_booleanv(name: GLenum) -> bool {
    unsafe {
        let mut value: GLboolean = 0;

        ffi::glGetBooleanv(name, &mut value);

        value == GL_TRUE
    }
}

pub fn get_buffer_parameteriv(target: GLenum, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetBufferParameteriv(target, name, &mut value);

        value
    }
}

pub fn get_error() -> GLenum {
    unsafe {
        ffi::glGetError()
    }
}

pub fn get_floatv(name: GLenum) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetFloatv(name, &mut value);

        value
    }
}

pub fn get_framebuffer_attachment_parameteriv(target: GLenum, attachment: GLenum,
                                              name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetFramebufferAttachmentParameteriv(target, attachment, name, &mut value);

        value
    }
}

pub fn get_integerv(name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetIntegerv(name, &mut value);

        value
    }
}

pub fn get_programiv(program: GLuint, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetProgramiv(program, name, &mut value);

        value
    }
}

pub fn get_program_info_log(program: GLuint, max_length: GLsizei) -> Option<String> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut log = String::with_capacity(max_length as usize);

        ffi::glGetProgramInfoLog(program, max_length, &mut length,
                                 log.as_mut_vec().as_mut_ptr() as *mut i8);

        if length > 0 {
            log.as_mut_vec().set_len(length as usize);
            log.truncate(length as usize);

            Some(log)
        } else {
            None
        }
    }
}

pub fn get_renderbuffer_parameteriv(target: GLenum, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetRenderbufferParameteriv(target, name, &mut value);

        value
    }
}

pub fn get_shaderiv(shader: GLuint, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetShaderiv(shader, name, &mut value);

        value
    }
}

pub fn get_shader_info_log(shader: GLuint, max_length: GLsizei) -> Option<String> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut log = String::with_capacity(max_length as usize);

        ffi::glGetShaderInfoLog(shader, max_length, &mut length,
                                log.as_mut_vec().as_mut_ptr() as *mut i8);

        if length > 0 {
            log.as_mut_vec().set_len(length as usize);
            log.truncate(length as usize);

            Some(log)
        } else {
            None
        }
    }
}

pub fn get_shader_precision_format(shader_type: GLenum,
                                   precision_type: GLenum) -> ShaderPrecisionFormat {
    unsafe {
        let mut precision: GLint = 0;
        let mut range:     [GLint; 2] = [0,0];

        ffi::glGetShaderPrecisionFormat(shader_type, precision_type, range.as_mut_ptr(),
                                        &mut precision);

        ShaderPrecisionFormat{ precision: precision,
                               range:     range }
    }
}

pub fn get_shader_source(shader: GLuint, max_length: GLsizei) -> Option<String> {
    unsafe {
        let mut length: GLsizei = 0;
        let mut source = String::with_capacity(max_length as usize);

        ffi::glGetShaderSource(shader, max_length, &mut length,
                               source.as_mut_vec().as_mut_ptr() as *mut GLchar);

        if length > 0 {
            source.as_mut_vec().set_len(length as usize);
            source.truncate(length as usize);

            Some(source)
        } else {
            None
        }
    }
}

pub fn get_string(name: GLenum) -> Option<String> {
    unsafe {
        let c_str = ffi::glGetString(name);

        if !c_str.is_null() {
            match from_utf8(CStr::from_ptr(c_str).to_bytes()) {
                Ok(s)  => Some(s.to_string()),
                Err(_) => None
            }
        } else {
            None
        }
    }
}

pub fn get_tex_parameterfv(target: GLenum, name: GLenum) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetTexParameterfv(target, name, &mut value);

        value
    }
}

pub fn get_tex_parameteriv(target: GLenum, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetTexParameteriv(target, name, &mut value);

        value
    }
}

pub fn get_uniformfv(program: GLuint, location: GLint) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetUniformfv(program, location, &mut value);

        value
    }
}

pub fn get_uniformiv(program: GLuint, location: GLint) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetUniformiv(program, location, &mut value);

        value
    }
}

pub fn get_uniform_location(program: GLuint, name: &str) -> GLint {
    unsafe {
        let c_str = CString::new(name).unwrap();
        ffi::glGetUniformLocation(program, c_str.as_ptr() as *const c_char)
    }
}

pub fn get_vertex_attribfv(index: GLuint, name: GLenum) -> GLfloat {
    unsafe {
        let mut value: GLfloat = 0.0;

        ffi::glGetVertexAttribfv(index, name, &mut value);

        value
    }
}

pub fn get_vertex_attribiv(index: GLuint, name: GLenum) -> GLint {
    unsafe {
        let mut value: GLint = 0;

        ffi::glGetVertexAttribiv(index, name, &mut value);

        value
    }
}

/*
pub fn get_vertex_attrib_pointerv(index: GLuint, name: GLenum, pointer: &mut &mut GLvoid) {
    unsafe {
    }
}
*/

pub fn hint(target: GLenum, mode: GLenum) {
    unsafe {
        ffi::glHint(target, mode)
    }
}

pub fn is_buffer(buffer: GLuint) -> bool {
    unsafe {
        ffi::glIsBuffer(buffer) == GL_TRUE
    }
}

pub fn is_enabled(feature: GLenum) -> bool {
    unsafe {
        ffi::glIsEnabled(feature) == GL_TRUE
    }
}

pub fn is_framebuffer(framebuffer: GLuint) -> bool {
    unsafe {
        ffi::glIsFramebuffer(framebuffer) == GL_TRUE
    }
}

pub fn is_program(program: GLuint) -> bool {
    unsafe {
        ffi::glIsProgram(program) == GL_TRUE
    }
}

pub fn is_renderbuffer(renderbuffer: GLuint) -> bool {
    unsafe {
        ffi::glIsRenderbuffer(renderbuffer) == GL_TRUE
    }
}

pub fn is_shader(shader: GLuint) -> bool {
    unsafe {
        ffi::glIsShader(shader) == GL_TRUE
    }
}

pub fn is_texture(texture: GLuint) -> bool {
    unsafe {
        ffi::glIsTexture(texture) == GL_TRUE
    }
}

pub fn line_width(width: GLfloat) {
    unsafe {
        ffi::glLineWidth(width)
    }
}

pub fn link_program(program: GLuint) {
    unsafe {
        ffi::glLinkProgram(program)
    }
}

pub fn pixel_storei(name: GLenum, param: GLint) {
    unsafe {
        ffi::glPixelStorei(name, param)
    }
}

pub fn polygon_offset(factor: GLfloat, units: GLfloat) {
    unsafe {
        ffi::glPolygonOffset(factor, units)
    }
}

pub fn read_pixels<T>(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum,
                      type_: GLenum, buffer: &mut [T]) {
    unsafe {
        ffi::glReadPixels(x, y, width, height, format, type_, buffer.as_mut_ptr() as *mut GLvoid)
    }
}

pub fn read_pixels_rgba(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> Vec<u8> {
    unsafe {
        let mut buffer: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

        ffi::glReadPixels(x, y, width, height, GL_RGBA, GL_UNSIGNED_BYTE,
                          buffer.as_mut_ptr() as *mut GLvoid);

        buffer.set_len((width * height * 4) as usize);
        buffer
    }
}

pub fn release_shader_compiler() {
    unsafe {
        ffi::glReleaseShaderCompiler()
    }
}

pub fn renderbuffer_storage(target: GLenum, internal_format: GLenum, width: GLsizei,
                            height: GLsizei) {
    unsafe {
        ffi::glRenderbufferStorage(target, internal_format, width, height)
    }
}

pub fn sample_coverage(value: GLclampf, invert: bool) {
    unsafe {
        ffi::glSampleCoverage(value, invert as GLboolean)
    }
}

pub fn scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        ffi::glScissor(x, y, width, height)
    }
}

pub fn shader_binary<T>(shaders: &[GLuint], data_format: GLenum, data: &[T], length: GLsizei) {
    unsafe {
    	let ptr = if data.len() == 0 { 0 as *const GLvoid }
    				else { data.as_ptr() as *const GLvoid };

        ffi::glShaderBinary(shaders.len() as GLsizei, shaders.as_ptr(), data_format, ptr, length)
    }
}

pub fn shader_source(shader: GLuint, source: &[u8]) {
    unsafe {
        let length: GLsizei = source.len() as GLsizei;

        ffi::glShaderSource(shader, 1, &(source.as_ptr() as *const GLchar), &length)
    }
}

pub fn stencil_func(func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe {
        ffi::glStencilFunc(func, ref_, mask)
    }
}

pub fn stencil_func_separate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe {
        ffi::glStencilFuncSeparate(face, func, ref_, mask)
    }
}

pub fn stencil_mask(mask: GLuint) {
    unsafe {
        ffi::glStencilMask(mask)
    }
}

pub fn stencil_mask_separate(face: GLenum, mask: GLuint) {
    unsafe {
        ffi::glStencilMaskSeparate(face, mask)
    }
}

pub fn stencil_op(fail: GLenum, z_fail: GLenum, z_pass: GLenum) {
    unsafe {
        ffi::glStencilOp(fail, z_fail, z_pass)
    }
}

pub fn stencil_op_separate(face: GLenum, fail: GLenum, z_fail: GLenum, z_pass: GLenum) {
    unsafe {
        ffi::glStencilOpSeparate(face, fail, z_fail, z_pass)
    }
}

pub fn tex_image_2d<T>(target: GLenum, level: GLint, internal_format: GLint, width: GLsizei,
                       height: GLsizei, border: GLint, format: GLenum, type_: GLenum,
                       buffer: &[T]) {
    unsafe {
    	let ptr = if buffer.len() == 0 { 0 as *const GLvoid }
    				else { buffer.as_ptr() as *const GLvoid };

        ffi::glTexImage2D(target, level, internal_format, width, height, border, format, type_, ptr)
    }
}

pub fn tex_parameterf(target: GLenum, name: GLenum, value: GLfloat) {
    unsafe {
        ffi::glTexParameterf(target, name, value)
    }
}

pub fn tex_parameterfv(target: GLenum, name: GLenum, value: &GLfloat) {
    unsafe {
        ffi::glTexParameterfv(target, name, value)
    }
}

pub fn tex_parameteri(target: GLenum, name: GLenum, value: GLint) {
    unsafe {
        ffi::glTexParameteri(target, name, value)
    }
}

pub fn tex_parameteriv(target: GLenum, name: GLenum, value: &GLint) {
    unsafe {
        ffi::glTexParameteriv(target, name, value)
    }
}

pub fn tex_sub_image_2d<T>(target: GLenum, level: GLint, x_offset: GLint, y_offset: GLint,
                           width: GLsizei, height: GLsizei, format: GLenum,
                           type_: GLenum, buffer: &[T]) {
    unsafe {
    	let ptr = if buffer.len() == 0 { 0 as *const GLvoid }
    				else { buffer.as_ptr() as *const GLvoid };

        ffi::glTexSubImage2D(target, level, x_offset, y_offset, width, height, format, type_, ptr)
    }
}

pub fn uniform1f(location: GLint, x: GLfloat) {
    unsafe {
        ffi::glUniform1f(location, x)
    }
}

pub fn uniform1fv(location: GLint, values: &[GLfloat]) {
    unsafe {
        ffi::glUniform1fv(location, values.len() as GLsizei, values.as_ptr())
    }
}

pub fn uniform1i(location: GLint, x: GLint) {
    unsafe {
        ffi::glUniform1i(location, x)
    }
}

pub fn uniform1iv(location: GLint, values: &[GLint]) {
    unsafe {
        ffi::glUniform1iv(location, values.len() as GLsizei, values.as_ptr())
    }
}

pub fn uniform2f(location: GLint, x: GLfloat, y: GLfloat) {
    unsafe {
        ffi::glUniform2f(location, x, y)
    }
}

pub fn uniform2fv(location: GLint, values: &[GLfloat]) {
    unsafe {
        ffi::glUniform2fv(location, (values.len() / 2) as GLsizei, values.as_ptr())
    }
}

pub fn uniform2i(location: GLint, x: GLint, y: GLint) {
    unsafe {
        ffi::glUniform2i(location, x, y)
    }
}

pub fn uniform2iv(location: GLint, values: &[GLint]) {
    unsafe {
        ffi::glUniform2iv(location, (values.len() / 2) as GLsizei, values.as_ptr())
    }
}

pub fn uniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe {
        ffi::glUniform3f(location, x, y, z)
    }
}

pub fn uniform3fv(location: GLint, values: &[GLfloat]) {
    unsafe {
        ffi::glUniform3fv(location, (values.len() / 3) as GLsizei, values.as_ptr())
    }
}

pub fn uniform3i(location: GLint, x: GLint, y: GLint, z: GLint) {
    unsafe {
        ffi::glUniform3i(location, x, y, z)
    }
}

pub fn uniform3iv(location: GLint, values: &[GLint]) {
    unsafe {
        ffi::glUniform3iv(location, (values.len() / 3) as GLsizei, values.as_ptr())
    }
}

pub fn uniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe {
        ffi::glUniform4f(location, x, y, z, w)
    }
}

pub fn uniform4fv(location: GLint, values: &[GLfloat]) {
    unsafe {
        ffi::glUniform4fv(location, (values.len() / 4) as GLsizei, values.as_ptr())
    }
}

pub fn uniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {
    unsafe {
        ffi::glUniform4i(location, x, y, z, w)
    }
}

pub fn uniform4iv(location: GLint, values: &[GLint]) {
    unsafe {
        ffi::glUniform4iv(location, (values.len() / 4) as GLsizei, values.as_ptr())
    }
}

pub fn uniform_matrix2fv(location: GLint, transpose: bool, values: &[GLfloat]) {
    unsafe {
        ffi::glUniformMatrix2fv(location, (values.len() / 2) as GLsizei,
                                transpose as GLboolean,
                                values.as_ptr() as *const GLfloat)
    }
}

pub fn uniform_matrix3fv(location: GLint, transpose: bool, values: &[GLfloat]) {
    unsafe {
        ffi::glUniformMatrix3fv(location, (values.len() / 9) as GLsizei,
                                transpose as GLboolean,
                                values.as_ptr() as *const GLfloat)
    }
}

pub fn uniform_matrix4fv(location: GLint, transpose: bool, values: &[GLfloat]) {
    unsafe {
        ffi::glUniformMatrix4fv(location, (values.len() / 4) as GLsizei,
                                transpose as GLboolean,
                                values.as_ptr() as *const GLfloat)
    }
}

pub fn use_program(program: GLuint) {
    unsafe {
        ffi::glUseProgram(program)
    }
}

pub fn validate_program(program: GLuint) {
    unsafe {
        ffi::glValidateProgram(program)
    }
}

pub fn vertex_attrib1f(index: GLuint, x: GLfloat) {
    unsafe {
        ffi::glVertexAttrib1f(index, x)
    }
}

pub fn vertex_attrib1fv(index: GLuint, values: &[GLfloat]) {
    unsafe {
        ffi::glVertexAttrib1fv(index, values.as_ptr())
    }
}

pub fn vertex_attrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    unsafe {
        ffi::glVertexAttrib2f(index, x, y)
    }
}

pub fn vertex_attrib2fv(index: GLuint, values: &[GLfloat]) {
    unsafe {
        ffi::glVertexAttrib2fv(index, values.as_ptr())
    }
}

pub fn vertex_attrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe {
        ffi::glVertexAttrib3f(index, x, y, z)
    }
}

pub fn vertex_attrib3fv(index: GLuint, values: &[GLfloat]) {
    unsafe {
        ffi::glVertexAttrib3fv(index, values.as_ptr())
    }
}

pub fn vertex_attrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe {
        ffi::glVertexAttrib4f(index, x, y, z, w)
    }
}

pub fn vertex_attrib4fv(index: GLuint, values: &[GLfloat]) {
    unsafe {
        ffi::glVertexAttrib4fv(index, values.as_ptr())
    }
}

pub fn vertex_attrib_pointer<T>(index: GLuint, size: GLint, type_: GLenum,
                                normalized: bool, stride: GLsizei, buffer: &[T]) {
    unsafe {
    	let ptr = if buffer.len() == 0 { 0 as *const GLvoid }
    				else { buffer.as_ptr() as *const GLvoid };

        ffi::glVertexAttribPointer(index, size, type_, normalized as GLboolean, stride, ptr)
    }
}

pub fn vertex_attrib_pointer_offset(index: GLuint, size: GLint, type_: GLenum,
                                    normalized: bool, stride: GLsizei, offset: GLuint) {
    unsafe {
        ffi::glVertexAttribPointer(index, size, type_,
                                   normalized as GLboolean,
                                   stride, offset as *const GLvoid)
    }
}

pub fn viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        ffi::glViewport(x, y, width, height)
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

pub mod ffi {
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

        pub fn glCopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint,
                                y: GLint, width: GLsizei, height: GLsizei, border: GLint);

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

        pub fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;

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

        pub fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;

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

        pub fn glShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar,
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
