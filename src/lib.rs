use aegis_core::{Value, NativeFn};
use std::{collections::HashMap, ffi::CString, mem, ptr};

#[unsafe(no_mangle)]
pub extern "C" fn _aegis_register(map: &mut HashMap<String, NativeFn>) {
    // Init & Clear
    map.insert("gl_load".to_string(), gl_load);
    map.insert("gl_viewport".to_string(), gl_viewport);
    map.insert("gl_clear_color".to_string(), gl_clear_color);
    map.insert("gl_clear".to_string(), gl_clear);

    // Shaders
    map.insert("gl_create_shader".to_string(), gl_create_shader);
    map.insert("gl_shader_source".to_string(), gl_shader_source);
    map.insert("gl_compile_shader".to_string(), gl_compile_shader);
    map.insert("gl_create_program".to_string(), gl_create_program);
    map.insert("gl_attach_shader".to_string(), gl_attach_shader);
    map.insert("gl_link_program".to_string(), gl_link_program);
    map.insert("gl_use_program".to_string(), gl_use_program);
    map.insert("gl_delete_shader".to_string(), gl_delete_shader);

    // Buffers (VBO / VAO)
    map.insert("gl_gen_buffers".to_string(), gl_gen_buffers);
    map.insert("gl_bind_buffer".to_string(), gl_bind_buffer);
    map.insert("gl_buffer_data".to_string(), gl_buffer_data);
    map.insert("gl_gen_vertex_arrays".to_string(), gl_gen_vertex_arrays);
    map.insert("gl_bind_vertex_array".to_string(), gl_bind_vertex_array);

    // Attributs
    map.insert("gl_vertex_attrib_pointer".to_string(), gl_vertex_attrib_pointer);
    map.insert("gl_enable_vertex_attrib_array".to_string(), gl_enable_vertex_attrib_array);

    // Uniforms
    map.insert("gl_get_uniform_location".to_string(), gl_get_uniform_location);
    map.insert("gl_uniform_4f".to_string(), gl_uniform_4f);

    // Textures
    map.insert("gl_gen_textures".to_string(), gl_gen_textures);
    map.insert("gl_bind_texture".to_string(), gl_bind_texture);
    map.insert("gl_tex_image_2d".to_string(), gl_tex_image_2d);
    map.insert("gl_generate_mipmap".to_string(), gl_generate_mipmap);
    map.insert("gl_tex_parameter_i".to_string(), gl_tex_parameter_i);
    
    // Draw
    map.insert("gl_draw_arrays".to_string(), gl_draw_arrays);
}

// --- UTILITAIRE ---
fn extract_f32_vec(val: &Value) -> Result<Vec<f32>, String> {
    if let Value::List(rc_list) = val {
        let list = rc_list.borrow();
        let mut vec = Vec::with_capacity(list.len());
        for v in list.iter() {
            vec.push(v.as_float()? as f32);
        }
        Ok(vec)
    } else {
        Err("Expected a List of floats".into())
    }
}

fn extract_u8_vec(val: &Value) -> Result<Vec<u8>, String> {
    if let Value::List(rc_list) = val {
        let list = rc_list.borrow();
        let mut vec = Vec::with_capacity(list.len());
        for v in list.iter() {
            vec.push(v.as_int()? as u8);
        }
        Ok(vec)
    }
    else {
        Err("Expected a List of integers (bytes)".into())
    }
}

// --- INIT ---
fn gl_load(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Expects loader pointer (int)".into());
    }

    let addr = args[0].as_int()? as usize;

    type GlfwGetProcAddrFn = unsafe extern "C" fn(*const i8) -> *const std::ffi::c_void;

    let loader: GlfwGetProcAddrFn = unsafe {
        std::mem::transmute(addr)
    };

    gl::load_with(|symbol| {
        let c_str = CString::new(symbol).unwrap();
        unsafe { loader(c_str.as_ptr()) }
    });

    println!("[Rust-GL] OpenGL functions loaded via shared pointer.");
    Ok(Value::Boolean(true))
}

fn gl_viewport(args: Vec<Value>) -> Result<Value, String> {
    unsafe { 
        gl::Viewport(
            args[0].as_int()? as i32, 
            args[1].as_int()? as i32, 
            args[2].as_int()? as i32, 
            args[3].as_int()? as i32
        ); 
    }
    Ok(Value::Null)
}

fn gl_clear_color(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("Args: r, g, b, a (floats)".into());
    }

    let r = args[0].as_float()? as f32;
    let g = args[1].as_float()? as f32;
    let b = args[2].as_float()? as f32;
    let a = args[3].as_float()? as f32;

    unsafe {
        gl::ClearColor(r, g, b, a);
    }
    Ok(Value::Null)
}

fn gl_clear(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Args: mask (int)".into());
    }
    let mask = args[0].as_int()? as u32;

    unsafe {
        gl::Clear(mask);
    }
    Ok(Value::Null)
}


// --- SHADERS ---
fn gl_create_shader(args: Vec<Value>) -> Result<Value, String> {
    let type_ = args[0].as_int()? as u32;
    let id = unsafe {
        gl::CreateShader(type_)
    };
    Ok(Value::Integer(id as i64))
}

fn gl_shader_source(args: Vec<Value>) -> Result<Value, String> {
    let id = args[0].as_int()? as u32;
    let src = args[1].as_str()?;
    let c_src = CString::new(src).unwrap();
    unsafe {
        gl::ShaderSource(id, 1, &c_src.as_ptr(), ptr::null());
    }
    Ok(Value::Null)
}

fn gl_compile_shader(args: Vec<Value>) -> Result<Value, String> {
    let id = args[0].as_int()? as u32;
    unsafe {
        gl::CompileShader(id);
    }
    Ok(Value::Null)
}

fn gl_create_program(_: Vec<Value>) -> Result<Value, String> {
    let id = unsafe {
        gl::CreateProgram()
    };
    Ok(Value::Integer(id as i64))
}

fn gl_attach_shader(args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        gl::AttachShader(
            args[0].as_int()? as u32,
            args[1].as_int()? as u32
        );
    }
    Ok(Value::Null)
}

fn gl_link_program(args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        gl::LinkProgram(args[0].as_int()? as u32);
    }
    Ok(Value::Null)
}

fn gl_use_program(args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        gl::UseProgram(args[0].as_int()? as u32);
    }
    Ok(Value::Null)
}

fn gl_delete_shader(args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        gl::DeleteShader(args[0].as_int()? as u32);
    }
    Ok(Value::Null)
}

// --- BUFFERS & VAO ---
fn gl_gen_vertex_arrays(_: Vec<Value>) -> Result<Value, String> {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    Ok(Value::Integer(vao as i64))
}

fn gl_bind_vertex_array(args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        gl::BindVertexArray(args[0].as_int()? as u32);
    }
    Ok(Value::Null)
}

fn gl_gen_buffers(_: Vec<Value>) -> Result<Value, String> {
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    Ok(Value::Integer(vbo as i64))
}

fn gl_bind_buffer(args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        gl::BindBuffer(args[0].as_int()? as u32, args[1].as_int()? as u32);
    }
    Ok(Value::Null)
}

fn gl_buffer_data(args: Vec<Value>) -> Result<Value, String> {
    let target = args[0].as_int()? as u32;
    let data = extract_f32_vec(&args[1])?;
    let usage = args[2].as_int()? as u32;

    unsafe {
        gl::BufferData(
            target,
            (data.len() * mem::size_of::<f32>()) as isize,
            data.as_ptr() as *const _,
            usage
        );
    }
    Ok(Value::Null)
}

// --- ATTRIBUTS ---
fn gl_vertex_attrib_pointer(args: Vec<Value>) -> Result<Value, String> {
    let index = args[0].as_int()? as u32;
    let size = args[1].as_int()? as i32;
    let type_ = args[2].as_int()? as u32;
    let normalized = if args[3].as_bool()? { gl::TRUE } else { gl::FALSE } as u8;
    let stride = args[4].as_int()? as i32;
    let offset = args[5].as_int()? as usize; // Offset en octets

    unsafe {
        gl::VertexAttribPointer(
            index, size, type_, normalized,
            stride,
            offset as *const std::ffi::c_void
        );
    }
    Ok(Value::Null)
}

fn gl_enable_vertex_attrib_array(args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        gl::EnableVertexAttribArray(args[0].as_int()? as u32);
    }
    Ok(Value::Null)
}

// --- UNIFORMS ---
fn gl_get_uniform_location(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Args: program(int), name(string)".into());
    }

    let program = args[0].as_int()? as u32;
    let name = args[1].as_str()?;
    let c_name = CString::new(name).unwrap();

    let loc = unsafe { gl::GetUniformLocation(program, c_name.as_ptr()) };

    Ok(Value::Integer(loc as i64))
}

fn gl_uniform_4f(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 5 { 
        return Err("Args: location, x, y, z, w".into()); 
    }

    let loc = args[0].as_int()? as i32;
    let x = args[1].as_float()? as f32;
    let y = args[2].as_float()? as f32;
    let z = args[3].as_float()? as f32;
    let w = args[4].as_float()? as f32;

    unsafe {
        gl::Uniform4f(loc, x, y, z, w);
    }
    Ok(Value::Null)
}

// --- TEXTURES ---
fn gl_gen_textures(_: Vec<Value>) -> Result<Value, String> {
    let mut id = 0;
    unsafe { gl::GenTextures(1, &mut id); }
    Ok(Value::Integer(id as i64))
}

fn gl_bind_texture(args: Vec<Value>) -> Result<Value, String> {
    let target = args[0].as_int()? as u32;
    let id = args[1].as_int()? as u32;
    unsafe { gl::BindTexture(target, id); }
    Ok(Value::Null)
}

fn gl_tex_image_2d(args: Vec<Value>) -> Result<Value, String> {
    // Args: target, level, internal_format, width, height, border, format, type, data
    if args.len() != 9 { return Err("Args: target, level, internal, w, h, border, format, type, data".into()); }
    
    let target = args[0].as_int()? as u32;
    let level = args[1].as_int()? as i32;
    let internal = args[2].as_int()? as i32;
    let w = args[3].as_int()? as i32;
    let h = args[4].as_int()? as i32;
    let border = args[5].as_int()? as i32;
    let format = args[6].as_int()? as u32;
    let type_ = args[7].as_int()? as u32;
    
    // Conversion lourde mais nécessaire : List<Value> -> Vec<u8>
    let data = extract_u8_vec(&args[8])?;

    unsafe {
        gl::TexImage2D(
            target, level, internal, w, h, border, format, type_, 
            data.as_ptr() as *const std::ffi::c_void
        );
    }
    Ok(Value::Null)
}

fn gl_generate_mipmap(args: Vec<Value>) -> Result<Value, String> {
    unsafe { gl::GenerateMipmap(args[0].as_int()? as u32); }
    Ok(Value::Null)
}

fn gl_tex_parameter_i(args: Vec<Value>) -> Result<Value, String> {
    let target = args[0].as_int()? as u32;
    let pname = args[1].as_int()? as u32;
    let param = args[2].as_int()? as i32;
    unsafe { gl::TexParameteri(target, pname, param); }
    Ok(Value::Null)
}

// --- DRAW ---
fn gl_draw_arrays(args: Vec<Value>) -> Result<Value, String> {
    let mode = args[0].as_int()? as u32;
    let first = args[1].as_int()? as i32;
    let count = args[2].as_int()? as i32;
    unsafe { gl::DrawArrays(mode, first, count); }
    Ok(Value::Null)
}
