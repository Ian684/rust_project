#[derive(Clone,Debug)]
enum Exercise39State{Idle,Running,Finished}

struct Exercise39Record{id:usize,label:String,score:i32,state:Exercise39State}

impl Exercise39Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_39_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+39)).sum()
}

fn main(){
    println!("Exercise 39: hello1");
    let records=[
        Exercise39Record{id:1,label:"Ada".into(),score:98,state:Exercise39State::Finished},
        Exercise39Record{id:2,label:"Linus".into(),score:86,state:Exercise39State::Running},
        Exercise39Record{id:3,label:"Grace".into(),score:73,state:Exercise39State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_39_operation(&[12,7,19,3,25]));
}
