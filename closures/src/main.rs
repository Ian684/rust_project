#[derive(Clone,Debug)]
enum Exercise06State{Idle,Running,Finished}

struct Exercise06Record{id:usize,label:String,score:i32,state:Exercise06State}

impl Exercise06Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_06_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+6)).sum()
}

fn main(){
    println!("Exercise 06: closures");
    let records=[
        Exercise06Record{id:1,label:"Ada".into(),score:98,state:Exercise06State::Finished},
        Exercise06Record{id:2,label:"Linus".into(),score:86,state:Exercise06State::Running},
        Exercise06Record{id:3,label:"Grace".into(),score:73,state:Exercise06State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_06_operation(&[12,7,19,3,25]));
}
