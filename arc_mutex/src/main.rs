#[derive(Clone,Debug)]
enum Exercise01State{Idle,Running,Finished}

struct Exercise01Record{id:usize,label:String,score:i32,state:Exercise01State}

impl Exercise01Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_01_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+1)).sum()
}

fn main(){
    println!("Exercise 01: arc_mutex");
    let records=[
        Exercise01Record{id:1,label:"Ada".into(),score:98,state:Exercise01State::Finished},
        Exercise01Record{id:2,label:"Linus".into(),score:86,state:Exercise01State::Running},
        Exercise01Record{id:3,label:"Grace".into(),score:73,state:Exercise01State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_01_operation(&[12,7,19,3,25]));
}
