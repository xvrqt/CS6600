// Custom Error Type
pub mod error;
pub use error::InterfaceBlockError;
type Result<T> = std::result::Result<T, InterfaceBlockError>;

// OpenGL Types
use gl::types::*;
// Used for defining arrays of floats, vectors, and matrices
use std::ffi::CString;
use std::marker::PhantomData;
use std::rc::Rc;
use std::vec::Vec;

// Linear Algebra Crate -> Defining the Uniform and From<> traits on its types
// use ultraviolet::{Mat3, Mat4, Vec2, Vec3, Vec4};

// Add support for if you know the binding point ahead of time
pub struct Unattached<Value> {
    value: Vec<Value>,
}
pub struct Attached {
    buffer_id: GLuint,
    block_index: GLuint,
    binding_point: GLuint,
}

// Interface Block Buffer Types
pub struct Uniform;
pub struct Shader;

// Interface Block Buffer Packing Layout Types
pub struct Packed;
pub struct Shared;
pub struct Std140;
pub struct Std430;

// Implementations of different type combinations
pub struct InterfaceBlock<Uniform, Std140, Value, T> {
    name: CString,
    data: T,
    data_type: PhantomData<Value>,
    block_type: PhantomData<Uniform>,
    packing_type: PhantomData<Std140>,
}

// Public API for creating new, unattached Uniform Buffer Blocks
pub struct UniformBufferBlock;
impl UniformBufferBlock {
    pub fn new_std140<S, Value>(
        name: S,
        value: Vec<Value>,
    ) -> Result<InterfaceBlock<Uniform, Std140, Value, Unattached<Value>>>
    where
        S: AsRef<str>,
    {
        // Attempt CString conversion
        let name = CString::new(name.as_ref()).map_err(|_| {
            InterfaceBlockError::Other(crate::GLUtilityError::FailedToConvertToCString(
                name.as_ref().to_string(),
            ))
        })?;

        Ok(InterfaceBlock {
            name,
            data: Unattached { value },
            // Ghosts
            data_type: PhantomData,
            block_type: PhantomData,
            packing_type: PhantomData,
        })
    }
}

// Only std140 rep, only uniform for the current implementation
impl<T, Value> InterfaceBlock<Uniform, T, Value, Unattached<Value>> {
    pub(crate) fn attach(
        self,
        program_id: GLuint,
        binding_point: GLuint,
    ) -> Result<Rc<InterfaceBlock<Uniform, T, Value, Attached>>> {
        unsafe {
            gl::UseProgram(program_id);
        }
        let name = self.name.clone();

        // Get location of the lights on the GPU, and length of the array CPU side
        let block_index = get_interface_block_index(program_id, name.clone())?;

        // Initialize a Uniform Buffer for the lights, if we haven't alreaady
        let mut buffer_id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
        }

        // Buffer the data
        let ptr = self.data.value.as_ptr() as *const std::ffi::c_void;
        let size =
            (self.data.value.len() as u32 * std::mem::size_of::<Value>() as u32) as GLsizeiptr;
        unsafe {
            gl::UniformBlockBinding(buffer_id, block_index, binding_point);
            // Could be error site
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer_id);
            gl::BufferData(gl::UNIFORM_BUFFER, size, ptr, gl::DYNAMIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_point, buffer_id);
        }
        let attached = InterfaceBlock {
            name,
            data: Attached {
                buffer_id,
                block_index,
                binding_point,
            },
            // Ghosts
            data_type: PhantomData,
            block_type: PhantomData,
            packing_type: PhantomData,
        };

        Ok(Rc::from(attached))
    }

    // Looks up the uniform index using `name` then initializes that location with the data
    // contained in `value` and finally returns the attached version of the struct
    // pub(crate) fn attach(self, program_id: GLuint) -> Result<Rc<dyn UpdateUniform>> {
    //     unsafe {
    //         gl::UseProgram(program_id);
    //     }
    //     let name = self.name.clone();
    //
    //     // Perform the lookup ;::; -1 is OpenGL's operation failed error code
    //     let location: GLint;
    //     unsafe {
    //         location = gl::GetUniformLocation(program_id, name.into_raw());
    //     }
    //     if location == -1 {
    //         Err(UniformError::CouldNotFindUniformIndex(
    //             self.name.to_string_lossy().to_string(),
    //         ))
    //     } else {
    //         // Buffer the initial data to the uniform
    //         self.value.initialize(location);
    //         let attached = UniformHandle {
    //             location,
    //             value: std::marker::PhantomData::<Value>,
    //         };
    //         let attached: Rc<dyn UpdateUniform> = Rc::from(attached);
    //         Ok(attached)
    //     }
    // }
    //
    // // Convenience function that transforms the name from CString to Rc<str> to use as a key in a
    // // GLProgram's uniform HashMap
    // pub(crate) fn key(&self) -> Rc<str> {
    //     let name = self.name.to_string_lossy();
    //     let key: Rc<str> = Rc::from(name);
    //     key
    // }
}

// Similar to get_uniform_location but for block indices
fn get_interface_block_index(program_id: GLuint, name: CString) -> Result<GLuint> {
    let location;
    unsafe {
        location = gl::GetUniformBlockIndex(program_id, name.clone().into_raw());
    }
    match location {
        gl::INVALID_INDEX => Err(InterfaceBlockError::CouldNotFindUniformIndex(name)),
        _ => Ok(location),
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

// Implemented by types that can be passed to a shader using OpenGL's `glUniform*` functions
pub trait UniformValue {
    fn initialize(&self, location: GLint) -> ();
}
// Implemented by attached Uniform structs that call ".set()" on their inner values, passing in the
// uniform's location
pub trait UpdateUniform {
    // Updates the uniform at `location` to the value of self
    fn update(&self, value: &dyn UniformValue) -> ();
}
