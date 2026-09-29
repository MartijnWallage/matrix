use crate::Vector;

impl Vector<f32> {
   pub fn norm_1(&self) -> f32 {
       let mut acc = 0.0;

       for i in 0..self.len() {
           acc += self.get(i).abs();
       }

       acc
   } 

   pub fn norm(&self) -> f32 {
       self.dot(self).sqrt()
   }

   pub fn norm_inf(&self) -> f32 {
       let mut high = 0.0;

       for i in 0..self.len() {
           let k = self.get(i).abs();

           if k > high {
               high = k;
           }
       }
       
       high
   }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn test_norm_1() {
        let v = Vector::<f32>::from(vec![3.0, -4.0]);

        assert_eq!(v.norm_1(), 7.0);
        assert_eq!(v.norm(), 5.0);
        assert_eq!(v.norm_inf(), 4.0);
    }
}
