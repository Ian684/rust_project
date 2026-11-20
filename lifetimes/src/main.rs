#[derive(Clone,Debug)]
enum Exercise10State{Idle,Running,Finished}

struct Exercise10Record{id:usize,label:String,score:i32,state:Exercise10State}

impl Exercise10Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_10_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+10)).sum()
}

fn main(){
    println!("Exercise 10: lifetimes");
    let records=[
        Exercise10Record{id:1,label:"Ada".into(),score:98,state:Exercise10State::Finished},
        Exercise10Record{id:2,label:"Linus".into(),score:86,state:Exercise10State::Running},
        Exercise10Record{id:3,label:"Grace".into(),score:73,state:Exercise10State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_10_operation(&[12,7,19,3,25]));
}
