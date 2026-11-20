#[derive(Clone,Debug)]
enum Exercise23State{Idle,Running,Finished}

struct Exercise23Record{id:usize,label:String,score:i32,state:Exercise23State}

impl Exercise23Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_23_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+23)).sum()
}

fn main(){
    println!("Exercise 23: structs");
    let records=[
        Exercise23Record{id:1,label:"Ada".into(),score:98,state:Exercise23State::Finished},
        Exercise23Record{id:2,label:"Linus".into(),score:86,state:Exercise23State::Running},
        Exercise23Record{id:3,label:"Grace".into(),score:73,state:Exercise23State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_23_operation(&[12,7,19,3,25]));
}
