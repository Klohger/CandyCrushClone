use crate::component::Component;


//pub type Object = Vec<Component>;


pub struct Object {
    pub components : Vec<Box<dyn Component>>,

}
impl Object {
    pub unsafe fn get_component<T : Component>(this : *const Object, identifier: &str) -> Option<*mut T> {
        let mut t = None;
        for component in &(*this).components {
            let component = &**component as *const dyn Component as *mut dyn Component;
            if (*component).identifier() == identifier {
                t = Some(component as *mut T);
            }
        }
        return t;
    }
}
/*
macro_rules! get_component {
    ($object:expr,$pattern:path) => {
        {
            let mut val = None;
            for component in &mut (*$object).components {
                let component = component as *mut Component;
                
                if let $pattern = *component {
                    val = Some(component);
                }
            }
            val
        }
        
    };
}

pub(crate) use get_component;
 */
/*
pub trait ObjectTrait {
    unsafe fn update(&mut self, scene : *mut Scene, display : &Display) -> Option<NextScene>;
    fn draw(&self, scene : &Scene, frame : &mut Frame, display : &Display);
    //fn serialize(&self) -> Vec<u8>;
}

#[macro_export] macro_rules! objects {
    () => (
        $crate::__rust_force_expr!($crate::vec::Vec::<crate::object::Object>::new())
    );
    ($elem:expr; $n:expr) => (
        vec::<crate::object::Object>::from_elem($elem, $n)
    );
    ($($x:expr),* $(,)?) => (
        <[_]>::into_vec(Box::new([$(Box::new($x)),+]))
    );
}
*/