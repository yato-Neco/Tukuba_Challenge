#[test]
fn test() {
    let mut result: u32 = 0;
    unsafe {ShareLib::call_none_args("./adder.dll", "main", &mut result) };
    println!("{:x}", result);

    unsafe {ShareLib::call_some_args("./adder.dll", "some_args", 0xf_u32, &mut result) };
    println!("{:x}", result);
}

pub struct ShareLib {}

impl ShareLib {
    /// 共有ライブラリ呼び出し
    /// resultに入るデータが型を無視する場合があるため unsafe
    /// 引数なしver.
    /// 
    /// ```
    /// let mut result: u32 = 0;
    /// unsafe {ShareLib::call_none_args("./adder.dll", "none_args", &mut result) };
    /// println!("{:x}", result);
    /// ```
    pub unsafe fn call_none_args<T>(path: &str, func_name: &str, result: &mut T) {
        match libloading::Library::new(path) {
            Ok(lib) => {
                match lib
                    .get::<libloading::Symbol<unsafe extern "C" fn() -> T>>(func_name.as_bytes())
                {
                    Ok(func) => {
                        *result = func();
                    }
                    Err(_) => {
                        panic!("func get error");
                    }
                }
            }
            Err(_) => {
                panic!("lib link error");
            }
        }
    }

    /// 共有ライブラリ呼び出し
    /// resultに入るデータが型を無視する場合があるため unsafe
    /// 引数ありver.
    /// ```
    /// let mut result: u32 = 0;
    /// unsafe {ShareLib::call_some_args("./adder.dll", "some_args", 0_u32, &mut result) };
    /// println!("{:x}", result);
    /// ```
    pub unsafe fn call_some_args<T, I>(path: &str, func_name: &str, args: I, result: &mut T) {
        match libloading::Library::new(path) {
            Ok(lib) => {
                match lib
                    .get::<libloading::Symbol<unsafe extern "C" fn(I) -> T>>(func_name.as_bytes())
                {
                    Ok(func) => {
                        *result = func(args);
                    }
                    Err(_) => {
                        panic!("func get error");
                    }
                }
            }
            Err(_) => {
                panic!("lib link error");
            }
        }
    }
}
