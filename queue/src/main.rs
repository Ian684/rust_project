#[derive(Clone,Debug)]
enum Exercise16State{Idle,Running,Finished}

struct Exercise16Record{id:usize,label:String,score:i32,state:Exercise16State}

impl Exercise16Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_16_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+16)).sum()
}

fn main(){
    println!("Exercise 16: queue");
    let records=[
        Exercise16Record{id:1,label:"Ada".into(),score:98,state:Exercise16State::Finished},
        Exercise16Record{id:2,label:"Linus".into(),score:86,state:Exercise16State::Running},
        Exercise16Record{id:3,label:"Grace".into(),score:73,state:Exercise16State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_16_operation(&[12,7,19,3,25]));
}
