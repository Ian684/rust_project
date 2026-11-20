#[derive(Clone,Debug)]
enum Exercise38State{Idle,Running,Finished}

struct Exercise38Record{id:usize,label:String,score:i32,state:Exercise38State}

impl Exercise38Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_38_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+38)).sum()
}

fn main(){
    println!("Exercise 38: hello");
    let records=[
        Exercise38Record{id:1,label:"Ada".into(),score:98,state:Exercise38State::Finished},
        Exercise38Record{id:2,label:"Linus".into(),score:86,state:Exercise38State::Running},
        Exercise38Record{id:3,label:"Grace".into(),score:73,state:Exercise38State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_38_operation(&[12,7,19,3,25]));
}
