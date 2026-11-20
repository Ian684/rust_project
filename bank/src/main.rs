#[derive(Clone,Debug)]
enum Exercise31State{Idle,Running,Finished}

struct Exercise31Record{id:usize,label:String,score:i32,state:Exercise31State}

impl Exercise31Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_31_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+31)).sum()
}

fn main(){
    println!("Exercise 31: bank");
    let records=[
        Exercise31Record{id:1,label:"Ada".into(),score:98,state:Exercise31State::Finished},
        Exercise31Record{id:2,label:"Linus".into(),score:86,state:Exercise31State::Running},
        Exercise31Record{id:3,label:"Grace".into(),score:73,state:Exercise31State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_31_operation(&[12,7,19,3,25]));
}
