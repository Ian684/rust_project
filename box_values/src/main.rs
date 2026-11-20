#[derive(Clone,Debug)]
enum Exercise04State{Idle,Running,Finished}

struct Exercise04Record{id:usize,label:String,score:i32,state:Exercise04State}

impl Exercise04Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_04_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+4)).sum()
}

fn main(){
    println!("Exercise 04: box_values");
    let records=[
        Exercise04Record{id:1,label:"Ada".into(),score:98,state:Exercise04State::Finished},
        Exercise04Record{id:2,label:"Linus".into(),score:86,state:Exercise04State::Running},
        Exercise04Record{id:3,label:"Grace".into(),score:73,state:Exercise04State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_04_operation(&[12,7,19,3,25]));
}
