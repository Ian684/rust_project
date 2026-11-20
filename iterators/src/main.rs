#[derive(Clone,Debug)]
enum Exercise09State{Idle,Running,Finished}

struct Exercise09Record{id:usize,label:String,score:i32,state:Exercise09State}

impl Exercise09Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_09_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+9)).sum()
}

fn main(){
    println!("Exercise 09: iterators");
    let records=[
        Exercise09Record{id:1,label:"Ada".into(),score:98,state:Exercise09State::Finished},
        Exercise09Record{id:2,label:"Linus".into(),score:86,state:Exercise09State::Running},
        Exercise09Record{id:3,label:"Grace".into(),score:73,state:Exercise09State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_09_operation(&[12,7,19,3,25]));
}
