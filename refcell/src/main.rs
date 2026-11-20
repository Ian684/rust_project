#[derive(Clone,Debug)]
enum Exercise18State{Idle,Running,Finished}

struct Exercise18Record{id:usize,label:String,score:i32,state:Exercise18State}

impl Exercise18Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_18_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+18)).sum()
}

fn main(){
    println!("Exercise 18: refcell");
    let records=[
        Exercise18Record{id:1,label:"Ada".into(),score:98,state:Exercise18State::Finished},
        Exercise18Record{id:2,label:"Linus".into(),score:86,state:Exercise18State::Running},
        Exercise18Record{id:3,label:"Grace".into(),score:73,state:Exercise18State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_18_operation(&[12,7,19,3,25]));
}
