use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io,
    io::{
        prelude::*,
        Error as IoError, 
        /*TODO ErrorKind as IoErrorKind,*/
    },
    path::Path,
};

fn main() {
    
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

// fn constrain<F>(f: F) -> F
// where
//     F: for<'a> Fn<T>(&'a Path, T) -> Result<(), Box<Error + Send + Sync>>
// {
//     f
// }

pub fn prompt_write_object_to_file<'a, T: Debug, P>(object: &T, default_path: Option<P>) 
    -> Result<(), Box<Error + Send + Sync>>
    where
        P: AsRef<Path>,
        P: Debug,
        P: PartialEq
    {
    // let object_type = t!(object);
    
        // type:\n\
        // {#?}\n\
    println!("Would you like to save the following object to a file? Object:\n\
        {:#?}\n\
        Enter Y to save to '{:?}' \
        (Linux paths won't work on Windows and vice versa), \
        N for no, or otherwise enter a file path to save to: ",
        object, /*object_type, */default_path);
    // TODO: further investigate whether there are options for opening a GUI file explorer to save in a 
    // cross-platform compatible way.
        let get_input = {
            let reader = io::stdin();
            let input_text = String::new();
            let bytes_read = reader.read_line(&mut input_text).expect("failed to read line");
            (input_text, bytes_read)
        };
    
    let (input, _) = get_input;

    let path = Path::new(&input);

    let mut file = File::create(path).expect("Error creating file, \
        check that the file path entered is valid, e.g. '~/foo.txt'");
    // TODO: error handling could improve.
    
    // let write_cl = constrain(|input_path, object| {
    //     let mut file = File::create(input_path).expect("Error creating file, \
    //         check that the file path entered is valid, e.g. ~/foo.txt");
    //     // TODO: not sure how helpful this above and below error checks are; investigate.
    //     file.write_all(unsafe {any_as_u8_slice(&object)}).expect("error writing to file");
    //     // TODO: this file should probably be encrypted and password-protected.
    //     Ok(())
    // });
    
    if input == "N" {
        // do nothing
    } else if input == "Y" {
        let check_valid_default_path = {
            match default_path {
                Some(v) => v.to_string(),
                None => {
                    println!("The default path wasn't entered, please enter it now: ");
                    (default_path, _) = get_input();
                    default_path
                },
            }
        };

        while check_valid_default_path == None {
            check_valid_default_path
        }
        
        write_object_to_file(default_path, object).expect("Can't write {:#?} to {:#?}", object, 
            default_path_as_string);
        // TODO: should probably use better error handling than expect.
    } else {
        write_object_to_file(&input, object).expect("Can't write {:#?} to {:#?}", object, &input);
    }
    Ok(())
}

pub fn write_object_to_file<T, P>(input_path: P, object: T) -> Result<(), IoError>
    where P: AsRef<Path>,
{
    let mut file = File::create(input_path).expect("Error creating file, \
        check that the file path entered is valid, e.g. ~/foo.txt");
    // TODO: not sure how helpful this above and below error checks are; investigate.
    file.write_all(unsafe {any_as_u8_slice(&object)}).expect("error writing to file");
    // TODO: this file should probably be encrypted and password-protected.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_write_object_to_file() {
        assert!(write_object_to_file("foo", "~/foo.txt"), ())
    }
}