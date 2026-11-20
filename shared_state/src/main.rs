#[derive(Clone,Debug)]
enum Exercise19State{Idle,Running,Finished}

struct Exercise19Record{id:usize,label:String,score:i32,state:Exercise19State}

impl Exercise19Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_19_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+19)).sum()
}

fn main(){
    println!("Exercise 19: shared_state");
    let records=[
        Exercise19Record{id:1,label:"Ada".into(),score:98,state:Exercise19State::Finished},
        Exercise19Record{id:2,label:"Linus".into(),score:86,state:Exercise19State::Running},
        Exercise19Record{id:3,label:"Grace".into(),score:73,state:Exercise19State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_19_operation(&[12,7,19,3,25]));
}
