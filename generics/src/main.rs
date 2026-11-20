#[derive(Clone,Debug)]
enum Exercise28State{Idle,Running,Finished}

struct Exercise28Record{id:usize,label:String,score:i32,state:Exercise28State}

impl Exercise28Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_28_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+28)).sum()
}

fn main(){
    println!("Exercise 28: generics");
    let records=[
        Exercise28Record{id:1,label:"Ada".into(),score:98,state:Exercise28State::Finished},
        Exercise28Record{id:2,label:"Linus".into(),score:86,state:Exercise28State::Running},
        Exercise28Record{id:3,label:"Grace".into(),score:73,state:Exercise28State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_28_operation(&[12,7,19,3,25]));
}
