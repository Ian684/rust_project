#[derive(Clone,Debug)]
enum Exercise25State{Idle,Running,Finished}

struct Exercise25Record{id:usize,label:String,score:i32,state:Exercise25State}

impl Exercise25Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_25_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+25)).sum()
}

fn main(){
    println!("Exercise 25: traits");
    let records=[
        Exercise25Record{id:1,label:"Ada".into(),score:98,state:Exercise25State::Finished},
        Exercise25Record{id:2,label:"Linus".into(),score:86,state:Exercise25State::Running},
        Exercise25Record{id:3,label:"Grace".into(),score:73,state:Exercise25State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_25_operation(&[12,7,19,3,25]));
}
