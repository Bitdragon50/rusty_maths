fn main() {
    let line_1 = Vector::new(2.0, 1.0);
    println!("The vector is: {}", &line_1.display());
    println!("The vector's scale is: {}", &line_1.magnitude());
    let rotated_i = Vector::new(0.0 , 1.0);
    let rotated_j = Vector::new(-1.0, 0.0);
    let rotation_matrix = Matrix::new(rotated_i, rotated_j);
    println!("The new vector resulting from rotation is {:#?}", line_1.linear_transformation_v2(&rotation_matrix));

    let m1 = Matrix::new(Vector::new(1.0, 1.0), Vector::new(-2.0, 0.0));

    let m2 = Matrix::new(Vector::new(0.0, 1.0), Vector::new(2.0, 0.0));

    let m2m1 = Matrix::composite(&m2,&m1);
    println!("The composed matrix of m2 and m1 is: {}", m2m1.display());
    println!("The determinant of matrix of m1 is: {}", m1.determinant());
    println!("The determinant of matrix of m2 is: {}", m2.determinant());
    println!("The determinant of matrix of m2 and m1 is: {}", m2m1.determinant());


    let n1= Matrix::new(Vector::new(4.0, 2.0), Vector::new(7.0, 6.0)); 
    println!("The Reciprocal of matrix {} \nis:\n {}", n1.display(), n1.reciprocal().display());
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64
}

struct Matrix {
    i: Vector,
    j: Vector
}

impl Vector{
    fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }
    fn magnitude( &self ) -> f64 {
        ((self.x * self.x) + ( self.y * self.y )).powf(0.5)
    }
    fn display(&self) -> String {
        format!("\n|{:.1}|\n|{:.1}|", self.x , self.y)
    }
    fn add(&self, other: &Vector) -> Vector {
        Vector { x: (self.x + other.x), y: (self.y + other.y) }
    }
    fn subtract(&self, other: &Vector) -> Vector {
        Vector { x: (self.x - other.x), y: (self.y - other.y) }
    }
    fn scale(&self, scalar: f64) -> Vector {
        Vector { x: (self.x * scalar), y: ( self.y * scalar ) }
    }
    fn linear_transformation( &self, transformer: &Matrix) -> Vector {
        Vector { 
                x: (( self.x * transformer.i.x ) + ( self.y * transformer.j.x )), 
                y: (( self.x * transformer.i.y ) + ( self.y * transformer.j.y )) 
            }
    }
    fn from_matrix(matrix: &Matrix ) -> Vector {
        let x = matrix.i.x + matrix.j.x;
        let y = matrix.i.y + matrix.j.y;
        Vector { x , y }
    }
    fn linear_transformation_v2( &self, matrix: &Matrix) -> Vector {
        // *** This was Experimental *** //
        Vector::from_matrix(&matrix.scale_by_vector(self))
    }
}

impl Matrix {
    fn new(i: Vector , j: Vector) -> Self {
        Matrix { i , j  }
    }
    fn composite(first: &Matrix, second: &Matrix) -> Matrix {
        let composed_i = second.i.linear_transformation(first);
        let composed_j = second.j.linear_transformation(first);
        Matrix { i: composed_i , j: composed_j }        
    }
    fn display(&self) -> String {
        format!("\n| {:.1} {:.1} |\n| {:.1} {:.1} |", self.i.x , self.j.x, self.i.y , self.j.y)
    }
    fn scale_by_vector(&self, scalar: &Vector) -> Matrix {
        // *** This was Experimental *** //
        Matrix {
            i: self.i.scale(scalar.x),
            j: self.j.scale(scalar.y)
        }
    }
    fn scale_by_unit(&self, scalar: f64) -> Matrix {
        Matrix {
            i: self.i.scale(scalar),
            j: self.j.scale(scalar)        }

    }
    fn determinant(&self) -> f64 {
        let a = self.i.x;
        let c = self.i.y;
        let b = self.j.x;
        let d = self.j.y;
        (a*d) - (c*b)
    }
    fn reciprocal(&self) -> Matrix {
        let scaled = self.scale_by_unit(1.0/self.determinant());
        let a = scaled.i.x;
        let c = scaled.i.y;
        let b = scaled.j.x;
        let d = scaled.j.y;
        Matrix::new(Vector::new(d, -c), Vector::new(-b, a))
    }
}