use std::ops::{Index,IndexMut};
#[derive(Clone)]
pub struct Vec2d<T> {
	pub items: Vec<T>,
	x_len: usize,
	y_len: usize,
}
impl <T>Vec2d<T> {
	pub fn new(x_len:usize,y_len:usize) -> Self {
		Vec2d {
			items : Vec::with_capacity(x_len * y_len),
			x_len: x_len,
			y_len: y_len,
		}
	}
	fn calculate_index(&self,index:(usize,usize)) -> Option<usize> {
		let (y_index,x_index) = index;
		let result;
		if (x_index < self.x_len) && (y_index < self.y_len) {
			result = Some(y_index * self.x_len + x_index);				
		} else {
			result = None;
		}
		return result;
	}
}
impl<T> Index<(usize,usize)> for Vec2d<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: (usize,usize)) -> &T {
    	let index = self.calculate_index(index);
    	match index {
    		Some(index) => {
		        return self.items.index(index);    			
    		},
    		None => {
    			panic!("Error while indexing Vec2d. Index out of bound.");
    		}
    	}
    }
}

impl<T> IndexMut<(usize,usize)> for Vec2d<T> {
    #[inline]
    fn index_mut(&mut self, index: (usize,usize)) -> &mut T {        
        let index = self.calculate_index(index);
    	match index {
    		Some(index) => {
		        return self.items.index_mut(index);    			
    		},
    		None => {
    			panic!("Error while indexing Vec2d. Index out of bound.");
    		}
    	}
    }
}

#[test]
fn it_works() {
}
