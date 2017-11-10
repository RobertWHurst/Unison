use std::io::Read;
use std::fs::OpenOptions;
use std::env;
use std::mem;
use std::path::Path;
use value::Value;

impl Value {
  pub(crate) fn eval(&mut self, template_recursion_level: usize) {
    if template_recursion_level > 10000 {
      return;
    }

    let value = {
      let expression = match *self {
        Value::HashMap(ref mut h) => return h.iter_mut().for_each(|(_, v)| v.eval(0)),
        Value::String(ref s) => s,
        _ => return,
      };

      if !expression.contains("$") {
        return;
      }

      let mut string = String::new();
      let mut env_var = String::new();
      let mut file_var = String::new();

      let mut start_var = false;
      let mut in_env_var = false;
      let mut in_file_var = false;
      let mut in_escape = false;
      let mut used_tmpl = false;

      for c in expression.chars() {
        // Start Var
        if !in_escape && c == '$' {
          if start_var {
            string.push('$');
          }
          start_var = true;
        }
        // Figure out kind of var
        else if start_var && (c == '{' || c == '(') {
          in_env_var = c == '{';
          in_file_var = c == '(';
          start_var = false;
        }
        // Template env var
        else if in_env_var {
          if !in_escape && c == '}' {
            match env::var(&env_var) {
              Ok(s) => string += &s,
              Err(_) => (),
            };
            in_env_var = false;
          } else {
            env_var.push(c);
            used_tmpl = true;
          }
        }
        // Template file var
        else if in_file_var {
          if !in_escape && c == ')' {
            let path = Path::new(&file_var);
            if path.is_file() {
              match OpenOptions::new().read(true).open(path) {
                Ok(mut f) => {
                  let _ = f.read_to_string(&mut string);
                }
                Err(_) => (),
              };
            }
            in_file_var = false;
          } else {
            file_var.push(c);
            used_tmpl = true;
          }
        }
        // Set escape char
        else if c == '\\' {
          if in_escape {
            string.push('\\');
            string.push('\\');
            in_escape = false;
          } else {
            if start_var {
              string.push('$');
              start_var = false;
            }
            in_escape = true;
          }
        } else {
          if in_escape {
            // Note: This is likely a good place for an invalid escape error
            in_escape = false;
          } else if start_var {
            string.push('$');
            string.push(c);
            start_var = false;
          } else {
            string.push(c);
          }
        }
      }
      if start_var {
        string.push('$');
      }

      let mut value = Value::String(string);
      if used_tmpl {
        value.eval(template_recursion_level + 1);
      }
      value
    };

    mem::replace(self, value);
  }
}
