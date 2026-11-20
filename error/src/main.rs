#[derive(Clone,Debug)]
enum Exercise34State{Idle,Running,Finished}

struct Exercise34Record{id:usize,label:String,score:i32,state:Exercise34State}

impl Exercise34Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_34_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+34)).sum()
}

fn main(){
    println!("Exercise 34: error");
    let records=[
        Exercise34Record{id:1,label:"Ada".into(),score:98,state:Exercise34State::Finished},
        Exercise34Record{id:2,label:"Linus".into(),score:86,state:Exercise34State::Running},
        Exercise34Record{id:3,label:"Grace".into(),score:73,state:Exercise34State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_34_operation(&[12,7,19,3,25]));
}
