pub mod error;
pub use error::UniformError;

use gl::types::*;

use bitflags::bitflags;
use std::vec::Vec;

// Flags that are used to set 'magic' uniforms such as 'time' or 'mouse position'
// During the render loop, the program will check which flags are set
// and update the corresponding uniform values appropriately
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct MagicUniform: u8 {
        const NONE = 0;
        const TIME = 1;
        const RESOLUTION = 1 << 1;
    }
}

// Uniform Types
pub trait Uniform {
    fn set(&mut self, location: GLint) -> Result<(), UniformError>;
}

// Values
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL1F(pub GLfloat);
impl Uniform for GL1F {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform1f(location, self.0);
        }
        Ok(())
    }
}
impl From<f32> for GL1F {
    fn from(value: f32) -> Self {
        GL1F(value)
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL2F(pub GLfloat, pub GLfloat);
impl Uniform for GL2F {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform2f(location, self.0, self.1);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL3F(pub GLfloat, pub GLfloat, pub GLfloat);
impl Uniform for GL3F {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform3f(location, self.0, self.1, self.2);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL4F(pub GLfloat, pub GLfloat, pub GLfloat, pub GLfloat);
impl Uniform for GL4F {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform4f(location, self.0, self.1, self.2, self.3);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL1I(pub GLint);
impl Uniform for GL1I {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform1i(location, self.0);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL2I(pub GLint, pub GLint);
impl Uniform for GL2I {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform2i(location, self.0, self.1);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL3I(pub GLint, pub GLint, pub GLint);
impl Uniform for GL3I {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform3i(location, self.0, self.1, self.2);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL4I(pub GLint, pub GLint, pub GLint, pub GLint);
impl Uniform for GL4I {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform4i(location, self.0, self.1, self.2, self.3);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL1U(pub GLuint);
impl Uniform for GL1U {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform1ui(location, self.0);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL2U(pub GLuint, pub GLuint);
impl Uniform for GL2U {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform2ui(location, self.0, self.1);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL3U(pub GLuint, pub GLuint, pub GLuint);
impl Uniform for GL3U {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform3ui(location, self.0, self.1, self.2);
        }
        Ok(())
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct GL4U(pub GLuint, pub GLuint, pub GLuint, pub GLuint);
impl Uniform for GL4U {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        unsafe {
            gl::Uniform4ui(location, self.0, self.1, self.2, self.3);
        }
        Ok(())
    }
}

/////////////
// Vectors //
/////////////
pub struct GL1FV(pub Vec<GL1F>);
impl Uniform for GL1FV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLfloat;
        unsafe {
            gl::Uniform1fv(location, count, ptr);
        }
        Ok(())
    }
}

pub struct GL2FV(pub Vec<GL2F>);
impl Uniform for GL2FV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLfloat;
        unsafe {
            gl::Uniform2fv(location, count, ptr);
        }
        Ok(())
    }
}

pub struct GL3FV(pub Vec<GL3F>);
impl Uniform for GL3FV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLfloat;
        unsafe {
            gl::Uniform3fv(location, count, ptr);
        }
        Ok(())
    }
}

pub struct GL4FV(pub Vec<GL4F>);
impl Uniform for GL4FV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLfloat;
        unsafe {
            gl::Uniform4fv(location, count, ptr);
        }
        Ok(())
    }
}

pub struct GL1IV(pub Vec<i32>);
impl Uniform for GL1IV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLint;
        unsafe {
            gl::Uniform1iv(location, count, ptr);
        }
        Ok(())
    }
}

pub struct GL2IV(pub Vec<(i32, i32)>);
impl Uniform for GL2IV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLint;
        unsafe {
            gl::Uniform2iv(location, count, ptr);
        }
        Ok(())
    }
}
pub struct GL3IV(pub Vec<(i32, i32, i32)>);
impl Uniform for GL3IV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLint;
        unsafe {
            gl::Uniform3iv(location, count, ptr);
        }
        Ok(())
    }
}
pub struct GL4IV(pub Vec<(i32, i32, i32, i32)>);
impl Uniform for GL4IV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLint;
        unsafe {
            gl::Uniform4iv(location, count, ptr);
        }
        Ok(())
    }
}

pub struct GL1UV(pub Vec<u32>);
impl Uniform for GL1UV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLuint;
        unsafe {
            gl::Uniform1uiv(location, count, ptr);
        }
        Ok(())
    }
}

pub struct GL2UV(pub Vec<(u32, u32)>);
impl Uniform for GL2UV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLuint;
        unsafe {
            gl::Uniform2uiv(location, count, ptr);
        }
        Ok(())
    }
}
pub struct GL3UV(pub Vec<(u32, u32, u32)>);
impl Uniform for GL3UV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLuint;
        unsafe {
            gl::Uniform3uiv(location, count, ptr);
        }
        Ok(())
    }
}
pub struct GL4UV(pub Vec<(u32, u32, u32, u32)>);
impl Uniform for GL4UV {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLuint;
        unsafe {
            gl::Uniform4uiv(location, count, ptr);
        }
        Ok(())
    }
}

//////////////
// Matrices //
//////////////

// 3x3 Matrix of Floats
pub struct GL3FM(
    pub  Vec<(
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
    )>,
);
impl Uniform for GL3FM {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLfloat;
        unsafe {
            gl::UniformMatrix3fv(location, count, gl::FALSE, ptr);
        }
        Ok(())
    }
}
impl TryFrom<Vec<f32>> for GL3FM {
    type Error = UniformError;
    fn try_from(mut v: Vec<f32>) -> Result<Self, Self::Error> {
        v.shrink_to_fit();
        // Check for conformity
        if v.len() % 9 != 0 {
            return Err(UniformError::MatrixConversion((3, 3)));
        }
        let mut gl3fm: Vec<(
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
        )> = Vec::new();
        while v.len() > 0 {
            let d: Vec<f32> = v.drain(0..9).collect();
            gl3fm.push((d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7], d[8]));
        }

        Ok(GL3FM(gl3fm))
    }
}

// 4x4 Matrix of Floats
pub struct GL4FM(
    pub  Vec<(
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
        GLfloat,
    )>,
);
impl Uniform for GL4FM {
    fn set(&mut self, location: GLint) -> Result<(), UniformError> {
        self.0.shrink_to_fit();
        let count: GLsizei = self
            .0
            .len()
            .try_into()
            .map_err(|_| UniformError::VectorLength)?;
        // Pointer to the vector's buffer
        let ptr = self.0.as_ptr() as *const GLfloat;
        unsafe {
            gl::UniformMatrix4fv(location, count, gl::FALSE, ptr);
        }
        Ok(())
    }
}
impl TryFrom<Vec<f32>> for GL4FM {
    type Error = UniformError;
    fn try_from(mut v: Vec<f32>) -> Result<Self, Self::Error> {
        v.shrink_to_fit();
        // Check for conformity
        if v.len() % 16 != 0 {
            return Err(UniformError::MatrixConversion((4, 4)));
        }
        let mut gl4fm: Vec<(
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
            GLfloat,
        )> = Vec::new();
        while v.len() > 0 {
            let d: Vec<f32> = v.drain(0..16).collect();
            gl4fm.push((
                d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7], d[8], d[9], d[10], d[11], d[12],
                d[13], d[14], d[15],
            ));
        }

        Ok(GL4FM(gl4fm))
    }
}
