//第一题
trait LampTime{
        fn getLampTime($self)-> i32;
}

enum StreetLamp{
    green,
    red,
    yellow,
}

impl LampTime for StreetLamp{
    fn getLampTime(&self) -> i32{
        match *self{
            StreetLamp::green => 360,
            StreetLamp:red => 370,
            _=> 380,
        }
    }
}

//第二题
fn calculationSum(ver: &Vec<u32>)-> Option<u32> {
        let mut sums: u32=0;
        for i in vec{
            match sums.checked_add(*1) {
                Some(s) => {sums=sums+s;}
                None => {return None;}
            };
        }
        Some(sums)
}

//第三题
trait CalculateArea{
        fn calculat(&self);
}

impl CalculateArea for triangle{
    fn calculat(&self) {
        let areas= self.bottom*self.high;
        print!("三角形面积 {}",areas);
    }
}

impl CalculateArea for square{
    fn calculat(&self){
        let areas= self.lengh*self.legth;
        print!("正方形面积{}",areas);
    }
}

struct triangle{
    bottom: u32,
    high: u32
}

struct square{
    length: u32
}

fn getArea<T:CalculateArea>(graphics: &T){
    graphics.calculat();
}