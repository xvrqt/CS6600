// Custom Error Type
pub mod error;
pub use error::UniformError;
type Result<T> = std::result::Result<T, UniformError>;

// OpenGL Types
use gl::types::*;
// Used for defining arrays of floats, vectors, and matrices
use std::ffi::CString;
use std::rc::Rc;
use std::vec::Vec;

// Used for 'MagicUniform' values to easily set and test for
use bitflags::bitflags;

// Linear Algebra Crate -> Defining the Uniform and From<> traits on its types
use ultraviolet::{Mat3, Mat4, Vec2, Vec3, Vec4};

// Flags that are used to set 'magic' uniforms such as 'time' or 'mouse position'
// During the render loop, the program will check which flags are set
// and update the corresponding uniform values appropriately
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MagicUniform: u8 {
        const NONE = 0;
        const TIME = 1;
        const RESOLUTION = 1 << 1;
    }
}

// Implemented by types that can be passed to a shader using OpenGL's `glUniform*` functions
// Essentially dynamic dispatch to the corrent OpenGL function
pub trait UniformValue {
    fn initialize(&self, location: GLint) -> ();
}
// Implemented by attached Uniform structs that call ".set()" on their inner values, passing in the
// uniform's location
pub trait UpdateUniform {
    // Updates the uniform at `location` to the value of self
    fn update(&self, value: &dyn UniformValue) -> ();
}

pub struct Uniform<'a, Value>
where
    Value: UniformValue,
{
    name: CString,
    value: &'a Value,
}

impl<'a, Value> Uniform<'a, Value>
where
    Value: UniformValue + 'static,
{
    pub fn new<S>(name: S, value: &'a Value) -> Result<Self>
    where
        S: AsRef<str>,
    {
        // Attempt CString conversion
        let name = CString::new(name.as_ref()).map_err(|_| {
            UniformError::Other(crate::GLUtilityError::FailedToConvertToCString(
                name.as_ref().to_string(),
            ))
        })?;

        Ok(Self { name, value })
    }

    // Looks up the uniform index using `name` then initializes that location with the data
    // contained in `value` and finally returns the attached version of the struct
    pub(crate) fn attach(self, program_id: GLuint) -> Result<Rc<dyn UpdateUniform>> {
        unsafe {
            gl::UseProgram(program_id);
        }
        let name = self.name.clone();

        // Perform the lookup ;::; -1 is OpenGL's operation failed error code
        let location: GLint;
        unsafe {
            location = gl::GetUniformLocation(program_id, name.into_raw());
        }
        if location == -1 {
            Err(UniformError::CouldNotFindUniformIndex(
                self.name.to_string_lossy().to_string(),
            ))
        } else {
            // Buffer the initial data to the uniform
            self.value.initialize(location);
            let attached = UniformHandle {
                location,
                value: std::marker::PhantomData::<Value>,
            };
            let attached: Rc<dyn UpdateUniform> = Rc::from(attached);
            Ok(attached)
        }
    }

    // Convenience function that transforms the name from CString to Rc<str> to use as a key in a
    // GLProgram's uniform HashMap
    pub(crate) fn key(&self) -> Rc<str> {
        let name = self.name.to_string_lossy();
        let key: Rc<str> = Rc::from(name);
        key
    }
}

// When a uniform value is attached to a GLProgram, it is transformed into this. It loses its
// `name` which becomes its key in the GLProgram's hashmap of Uniform values. It gains a definite
// location within the program.
// I really wish there was a good way to make this Uniform<Attached>
pub struct UniformHandle<Value>
where
    Value: UniformValue,
{
    // Which GLProgram Uniform Index this is bound to
    location: GLint,
    // The associated uniform type
    value: std::marker::PhantomData<Value>,
}

// The GLProgram's map is abstracted over the dynamic type that implements this trait
impl<Value> UpdateUniform for UniformHandle<Value>
where
    Value: UniformValue,
{
    // Simple wrapper around `.initialize()`
    fn update(&self, new_value: &dyn UniformValue) -> () {
        new_value.initialize(self.location)
    }
}

///////////////////////////////////////////////////////
// Trait UniformValue -> OpenGL glUniform*() Mapping //
///////////////////////////////////////////////////////

//////////////
// Matrices //
//////////////
impl UniformValue for Mat4 {
    fn initialize(&self, location: GLint) -> () {
        let ptr = self.cols.as_ptr() as *const GLfloat;
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, ptr);
        }
    }
}

impl UniformValue for Vec<Mat4> {
    fn initialize(&self, location: GLint) -> () {
        let ptr = self.as_ptr() as *const GLfloat;
        let length = self.len() as GLint;
        unsafe {
            gl::UniformMatrix4fv(location, length, gl::FALSE, ptr);
        }
    }
}

impl UniformValue for [Mat4] {
    fn initialize(&self, location: GLint) -> () {
        let ptr = self.as_ptr() as *const GLfloat;
        let length = self.len() as GLint;
        unsafe {
            gl::UniformMatrix4fv(location, length, gl::FALSE, ptr);
        }
    }
}

impl UniformValue for Mat3 {
    fn initialize(&self, location: GLint) -> () {
        let ptr = self.cols.as_ptr() as *const GLfloat;
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::FALSE, ptr);
        }
    }
}

impl UniformValue for Vec<Mat3> {
    fn initialize(&self, location: GLint) -> () {
        let ptr = self.as_ptr() as *const GLfloat;
        let length = self.len() as GLint;
        unsafe {
            gl::UniformMatrix3fv(location, length, gl::FALSE, ptr);
        }
    }
}

impl UniformValue for [Mat3] {
    fn initialize(&self, location: GLint) -> () {
        let ptr = self.as_ptr() as *const GLfloat;
        let length = self.len() as GLint;
        unsafe {
            gl::UniformMatrix3fv(location, length, gl::FALSE, ptr);
        }
    }
}

/////////////
// VECTORS //
/////////////
impl UniformValue for Vec4 {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform4f(location, self.x, self.y, self.z, self.w);
        }
    }
}

impl UniformValue for Vec<Vec4> {
    fn initialize(&self, location: GLint) -> () {
        let data = self.as_ptr() as *const GLfloat;
        let count = self.len() as GLint;
        unsafe {
            gl::Uniform4fv(location, count, data);
        }
    }
}

impl UniformValue for (f32, f32, f32, f32) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform4f(location, self.0, self.1, self.2, self.3);
        }
    }
}

impl UniformValue for (GLuint, GLuint, GLuint, GLuint) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform4ui(location, self.0, self.1, self.2, self.3);
        }
    }
}

impl UniformValue for (GLint, GLint, GLint, GLint) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform4i(location, self.0, self.1, self.2, self.3);
        }
    }
}

impl UniformValue for Vec3 {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform3f(location, self.x, self.y, self.z);
        }
    }
}

impl UniformValue for Vec<Vec3> {
    fn initialize(&self, location: GLint) -> () {
        let data = self.as_ptr() as *const GLfloat;
        let count = self.len() as GLint;
        unsafe {
            gl::Uniform3fv(location, count, data);
        }
    }
}

impl UniformValue for (f32, f32, f32) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform3f(location, self.0, self.1, self.2);
        }
    }
}

impl UniformValue for (GLuint, GLuint, GLuint) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform3ui(location, self.0, self.1, self.2);
        }
    }
}

impl UniformValue for (GLint, GLint, GLint) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform3i(location, self.0, self.1, self.2);
        }
    }
}

impl UniformValue for Vec2 {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform2f(location, self.x, self.y);
        }
    }
}

impl UniformValue for Vec<Vec2> {
    fn initialize(&self, location: GLint) -> () {
        let data = self.as_ptr() as *const GLfloat;
        let count = self.len() as GLint;
        unsafe {
            gl::Uniform2fv(location, count, data);
        }
    }
}

impl UniformValue for (f32, f32) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform2f(location, self.0, self.1);
        }
    }
}

impl UniformValue for (GLuint, GLuint) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform2ui(location, self.0, self.1);
        }
    }
}

impl UniformValue for (GLint, GLint) {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform2i(location, self.0, self.1);
        }
    }
}

////////////
// FLOATS //
////////////

impl UniformValue for f32 {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl UniformValue for Vec<f32> {
    fn initialize(&self, location: GLint) -> () {
        let data = self.as_ptr() as *const GLfloat;
        let count = self.len() as GLint;
        unsafe {
            gl::Uniform1fv(location, count, data);
        }
    }
}

//////////////
// INTEGERS //
//////////////

impl UniformValue for GLuint {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform1ui(location, *self);
        }
    }
}

impl UniformValue for Vec<GLuint> {
    fn initialize(&self, location: GLint) -> () {
        let data = self.as_ptr() as *const GLuint;
        let count = self.len() as GLint;
        unsafe {
            gl::Uniform1uiv(location, count, data);
        }
    }
}

impl UniformValue for GLint {
    fn initialize(&self, location: GLint) -> () {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl UniformValue for Vec<GLint> {
    fn initialize(&self, location: GLint) -> () {
        let data = self.as_ptr() as *const GLint;
        let count = self.len() as GLint;
        unsafe {
            gl::Uniform1iv(location, count, data);
        }
    }
}
