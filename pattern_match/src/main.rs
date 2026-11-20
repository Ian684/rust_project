#[derive(Clone,Debug)]
enum Exercise15State{Idle,Running,Finished}

struct Exercise15Record{id:usize,label:String,score:i32,state:Exercise15State}

impl Exercise15Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_15_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+15)).sum()
}

fn main(){
    println!("Exercise 15: pattern_match");
    let records=[
        Exercise15Record{id:1,label:"Ada".into(),score:98,state:Exercise15State::Finished},
        Exercise15Record{id:2,label:"Linus".into(),score:86,state:Exercise15State::Running},
        Exercise15Record{id:3,label:"Grace".into(),score:73,state:Exercise15State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_15_operation(&[12,7,19,3,25]));
}
