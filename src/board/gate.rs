#[derive(Clone)]
enum Gate {
    NoStorage2I1O {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        function: fn(Option<*const Option<bool>>, Option<*const Option<bool>>) -> Option<bool>,
        output: Option<bool>
    },
    NoStorageNI1O {
        i: Vec<*const Option<bool>>,
        function: fn(&Vec<*const Option<bool>>) -> Option<bool>,
        output: Option<bool>
    },
    NoStorage2INO {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        function: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
            &mut Box<[Option<bool>]>),
        output: Box<[Option<bool>]>
    },
    NoStorageNINO {
        i: Vec<*const Option<bool>>,
        function: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>),
        output: Box<[Option<bool>]>
    },
    Storage2I1O {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        storage: Box<[Option<bool>]>,
        function: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
            &mut Box<[Option<bool>]>) -> Option<bool>,
        output: Option<bool>
    },
    StorageNI1O {
        i: Vec<*const Option<bool>>,
        storage: Box<[Option<bool>]>,
        function: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>) -> Option<bool>,
        output: Option<bool>
    },
    Storage2INO {
        i1: Option<*const Option<bool>>,
        i2: Option<*const Option<bool>>,
        storage: Box<[Option<bool>]>,
        function: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
            &mut Box<[Option<bool>]>, &mut Box<[Option<bool>]>),
        output: Box<[Option<bool>]>
    },
    StorageNINO {
        i: Vec<*const Option<bool>>,
        storage: Box<[Option<bool>]>,
        function: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>,
            &mut Box<[Option<bool>]>),
        output: Box<[Option<bool>]>
    }
}

use Gate::*;

impl Gate {
    fn new_ns2i1o(i1: Option<*const Option<bool>>, i2: Option<*const Option<bool>>,
        function: fn(Option<*const Option<bool>>, Option<*const Option<bool>>)
            -> Option<bool>) -> Self {
        NoStorage2I1O {
            i1,
            i2,
            function,
            output: None
        }
    }

    fn new_nsni1o(i: Vec<*const Option<bool>>,
        function: fn(&Vec<*const Option<bool>>) -> Option<bool>) -> Self {
        NoStorageNI1O {
            i,
            function,
            output: None
        }
    }

    fn new_ns2ino(i1: Option<*const Option<bool>>, i2: Option<*const Option<bool>>,
        function: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
            &mut Box<[Option<bool>]>), output_slice_size: usize) -> Self {
        NoStorage2INO {
            i1,
            i2,
            function,
            output: vec![None; output_slice_size].into_boxed_slice()
        }
    }

    fn new_nsnino(i: Vec<*const Option<bool>>, output_slice_size: usize,
                  function: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>)) -> Self {
        NoStorageNINO {
            i,
            function,
            output: vec![None; output_slice_size].into_boxed_slice()
        }
    }

    fn new_s2i1o(i1: Option<*const Option<bool>>, i2: Option<*const Option<bool>>,
        storage_slice_size: usize, function: fn(Option<*const Option<bool>>,
            Option<*const Option<bool>>, &mut Box<[Option<bool>]>) -> Option<bool>) -> Self {
        Storage2I1O {
            i1,
            i2,
            storage: vec![None; storage_slice_size].into_boxed_slice(),
            function,
            output: None
        }
    }

    fn new_sni1o(i: Vec<*const Option<bool>>, storage_slice_size: usize,
        function: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>) -> Option<bool>) -> Self {
        StorageNI1O {
            i,
            storage: vec![None; storage_slice_size].into_boxed_slice(),
            function,
            output: None
        }
    }

    fn new_s2ino(i1: Option<*const Option<bool>>, i2: Option<*const Option<bool>>,
        storage_slice_size: usize, output_slice_size: usize,
        function: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
            &mut Box<[Option<bool>]>, &mut Box<[Option<bool>]>)) -> Self {
        Storage2INO {
            i1,
            i2,
            storage: vec![None; storage_slice_size].into_boxed_slice(),
            function,
            output: vec![None; output_slice_size].into_boxed_slice()
        }
    }

    fn new_snino(i: Vec<*const Option<bool>>, storage_slice_size: usize, output_slice_size: usize,
        function: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>, &mut Box<[Option<bool>]>))
        -> Self {
        StorageNINO {
            i,
            storage: vec![None; storage_slice_size].into_boxed_slice(),
            function,
            output: vec![None; output_slice_size].into_boxed_slice()
        }
    }

    fn set_i1(&mut self, new_i1: Option<*const Option<bool>>) {
        match self {
            NoStorage2I1O{i1, i2: _, function: _, output: _} => *i1 = new_i1,
            NoStorage2INO{i1, i2: _, function: _, output: _} => *i1 = new_i1,
            Storage2I1O{i1, i2: _, storage: _, function: _, output: _} => {
                *i1 = new_i1
            },
            Storage2INO{i1, i2: _, storage: _, function: _, output: _} => {
                *i1 = new_i1
            },
            _ => {}
        }
    }

    fn set_i2(&mut self, new_i2: Option<*const Option<bool>>) {
        match self {
            NoStorage2I1O{i1: _, i2, function: _, output: _} |
            NoStorage2INO{i1: _, i2, function: _, output: _} => *i2 = new_i2,
            Storage2I1O{i1: _, i2, storage: _, function: _, output: _} |
            Storage2INO{i1: _, i2, storage: _, function: _, output: _} => *i2 = new_i2,
            _ => {}
        }
    }

    fn set_i(&mut self, new_i: &[*const Option<bool>]) {
        match self {
            NoStorageNI1O{i, function: _, output: _} |
            NoStorageNINO{i, function: _, output: _} => {
                i.clear();
                i.extend_from_slice(new_i);
            },
            StorageNI1O{i, storage: _, function: _, output: _} |
            StorageNINO{i, storage: _, function: _, output: _} => {
                i.clear();
                i.extend_from_slice(new_i);
            },
            _ => {}
        }
    }

    fn get_output_ref_1o(&self) -> Option<*const Option<bool>> {
        match self {
            NoStorage2I1O{i1: _, i2: _, function: _, output} => Some(output),
            NoStorageNI1O{i: _, function: _, output} => Some(output),
            Storage2I1O{i1: _, i2: _, storage: _, function: _, output} => Some(output),
            StorageNI1O{i: _, storage: _, function: _, output} => Some(output),
            _ => None
        }
    }

    fn get_output_ref_no(&self) -> Option<*const Box<[Option<bool>]>> {
        match self {
            NoStorage2INO{i1: _, i2: _, function: _, output} => Some(output),
            NoStorageNINO{i: _, function: _, output} => Some(output),
            Storage2INO{i1: _, i2: _, storage: _, function: _, output} => Some(output),
            StorageNINO{i: _, storage: _, function: _, output} => Some(output),
            _ => None
        }
    }

    fn get_output_1o(&self) -> Option<bool> {
        match self {
            NoStorage2I1O{i1: _, i2: _, function: _, output} => *output,
            NoStorageNI1O{i: _, function: _, output} => *output,
            Storage2I1O{i1: _, i2: _, storage: _, function: _, output} => {
                *output
            },
            StorageNI1O{i: _, storage: _, function: _, output} => {
                *output
            },
            _ => None
        }
    }

    fn get_output_no(&self) -> Vec<Option<bool>> {
        match self {
            NoStorage2INO{i1: _, i2: _, function: _, output} => {
                output.iter().map(|&i| i).collect::<Vec<Option<bool>>>()
            },
            NoStorageNINO{i: _, function: _, output} => {
                output.iter().map(|&i| i).collect::<Vec<Option<bool>>>()
            },
            Storage2INO{i1: _, i2: _, storage: _, function: _, output} => {
                output.iter().map(|&i| i).collect::<Vec<Option<bool>>>()
            },
            StorageNINO{i: _, storage: _, function: _, output} => {
                output.iter().map(|&i| i).collect::<Vec<Option<bool>>>()
            },
            _ => Vec::new()
        }
    }

    fn set_ns2i1o_fn(&mut self, new_fn: fn(Option<*const Option<bool>>, Option<*const Option<bool>>)
        -> Option<bool>) {
        match self {
            NoStorage2I1O{i1: _, i2: _, function, output: _} => {
                *function = new_fn;
            },
            _ => {}
        }
    }

    fn set_nsni1o_fn(&mut self, new_fn: fn(&Vec<*const Option<bool>>) -> Option<bool>) {
        match self {
            NoStorageNI1O{i: _, function, output: _} => *function = new_fn,
            _ => {}
        }
    }

    fn set_ns2ino_fn(&mut self, new_fn: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
        &mut Box<[Option<bool>]>)) {
        match self {
            NoStorage2INO{i1: _, i2: _, function, output: _} => {
                *function = new_fn;
            },
            _ => {}
        }
    }

    fn set_nsnino_fn(&mut self, new_fn: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>)) {
        match self {
            NoStorageNINO{i: _, function, output: _} => {
                *function = new_fn;
            },
            _ => {}
        }
    }

    fn set_s2i1o_fn(&mut self, new_fn: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
        &mut Box<[Option<bool>]>) -> Option<bool>) {
        match self {
            Storage2I1O{i1: _, i2: _, storage: _, function, output: _} => {
                *function = new_fn;
            },
            _ => {}
        }
    }

    fn set_sni1o_fn(&mut self, new_fn: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>)
        -> Option<bool>) {
        match self {
            StorageNI1O{i: _, storage: _, function, output: _} => {
                *function = new_fn;
            },
            _ => {}
        }
    }

    fn set_s2ino_fn(&mut self, new_fn: fn(Option<*const Option<bool>>, Option<*const Option<bool>>,
        &mut Box<[Option<bool>]>, &mut Box<[Option<bool>]>)) {
        match self {
            Storage2INO{i1: _, i2: _, storage: _, function, output: _} => {
                *function = new_fn;
            },
            _ => {}
        }
    }

    fn set_snino_fn(&mut self, new_fn: fn(&Vec<*const Option<bool>>, &mut Box<[Option<bool>]>,
        &mut Box<[Option<bool>]>)) {
        match self {
            StorageNINO{i: _, storage: _, function, output: _} => {
                *function = new_fn;
            },
            _ => {}
        }
    }

    fn set_storage(&mut self, new_storage: Box<[Option<bool>]>) {
        match self {
            Storage2I1O{i1: _, i2: _, storage, function: _, output: _} => {
                *storage = new_storage;
            },
            StorageNI1O{i: _, storage, function: _, output: _} => {
                *storage = new_storage;
            },
            Storage2INO{i1: _, i2: _, storage, function: _, output: _} => {
                *storage = new_storage;
            },
            StorageNINO{i: _, storage, function: _, output: _} => {
                *storage = new_storage;
            },
            _ => {}
        }
    }

    fn update_inputs_2i(&mut self) {
        match self {
            NoStorage2I1O{i1, i2, function: _, output} => {
                if let (&mut Some(_), &mut Some(_)) = (i1, i2) {
                    *output = Some(false);
                } else {
                    *output = None;
                }
            },
            NoStorage2INO{i1, i2, function: _, output} => {
                if let (&mut Some(_), &mut Some(_)) = (i1, i2) {
                    for i in 0..output.len() {
                        output[i] = Some(false);
                    }
                } else {
                    for i in 0..output.len() {
                        output[i] = Some(false);
                    }
                }
            },
            Storage2I1O{i1, i2, storage: _, function: _, output} => {
                if let (&mut Some(_), &mut Some(_)) = (i1, i2) {
                    *output = Some(false);
                } else {
                    *output = None;
                }
            },
            Storage2INO{i1, i2, storage: _, function: _, output} => {
                if let (&mut Some(_), &mut Some(_)) = (i1, i2) {
                    for i in 0..output.len() {
                        output[i] = Some(false);
                    }
                } else {
                    for i in 0..output.len() {
                        output[i] = Some(false);
                    }
                }
            },
            _ => {}
        }
    }

    fn update_inputs_ni1o(&mut self, update_function: fn(&Vec<*const Option<bool>>)
        -> Option<bool>) {
        match self {
            NoStorageNI1O{i, function: _, output} => *output = update_function(&*i),
            StorageNI1O{i, storage: _, function: _, output} => *output = update_function(&*i),
            _ => {}
        }
    }

    fn update_inputs_nino(&mut self, update_function: fn(&Vec<*const Option<bool>>,
        &mut [Option<bool>])) {
        match self {
            NoStorageNINO{i, function: _, output} => update_function(i, output),
            StorageNINO{i, storage: _, function: _, output} => update_function(i, output),
            _ => {}
        }
    }

    fn eval(&mut self) {
        match self {
            NoStorage2I1O{i1, i2, function, output} => *output = function(*i1, *i2),
            NoStorageNI1O{i, function, output} => *output = function(&*i),
            NoStorage2INO{i1, i2, function, output} => function(*i1, *i2, output),
            NoStorageNINO{i, function, output} => function(&*i, output),
            Storage2I1O{i1, i2, storage, function, output} => *output = function(*i1, *i2, storage),
            StorageNI1O{i, storage, function, output} => *output = function(&*i, storage),
            Storage2INO{i1, i2, storage, function, output} => function(*i1, *i2, storage, output),
            StorageNINO{i, storage, function, output} => function(&*i, storage, output)
        }
    }
}