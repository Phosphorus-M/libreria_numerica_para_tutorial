#![doc = include_str!("../README.md")]

pub use prelude::add;
pub use prelude::Point;
pub mod prelude {
    //! # Prelude
    //! ## ¿Cual es la funcion de este modulo?
    //! Este modulo contiene funciones matematicas para realizar operaciones numericas.
    //! 
    //! ## Ejemplo
    //! ```rust
    //! # use libreria_numerica_para_tutorial::add;
    //! let result = add(2, 2);
    //! assert_eq!(result, 4);
    //! ```
    //! 
    //! Además, este modulo contiene la estructura [Point] que representa un punto en un plano cartesiano.
    //! 
    //! ## Ejemplo
    //! ```rust
    //! # use libreria_numerica_para_tutorial::Point;
    //! let point = Point { x: 0.0, y: 0.0 };
    //! ```

    /// Suma dos numeros
    /// ```rust
    /// # use libreria_numerica_para_tutorial::add;
    /// let result = add(2, 2);
    /// assert_eq!(result, 4);
    /// ```
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }
    
    /// Representa un punto en un plano cartesiano
    /// ```rust
    /// # use libreria_numerica_para_tutorial::Point;
    /// let point = Point { x: 0.0, y: 0.0 };
    /// ```
    /// 
    /// Los tipos de datos [f64] representan numeros de punto flotante de 64 bits.
    /// 
    /// ## TODO: 
    /// - Agregar soporte para numeros de punto flotante de 32 bits.
    /// - Agregar soporte para numeros de punto flotante de 128 bits.
    pub struct Point {
        pub x: f64,
        pub y: f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
