#[derive(Clone,Debug)]
enum Exercise05State{Idle,Running,Finished}

struct Exercise05Record{id:usize,label:String,score:i32,state:Exercise05State}

impl Exercise05Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_05_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+5)).sum()
}

fn main(){
    println!("Exercise 05: channels");
    let records=[
        Exercise05Record{id:1,label:"Ada".into(),score:98,state:Exercise05State::Finished},
        Exercise05Record{id:2,label:"Linus".into(),score:86,state:Exercise05State::Running},
        Exercise05Record{id:3,label:"Grace".into(),score:73,state:Exercise05State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_05_operation(&[12,7,19,3,25]));
}
