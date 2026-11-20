#[derive(Clone,Debug)]
enum Exercise36State{Idle,Running,Finished}

struct Exercise36Record{id:usize,label:String,score:i32,state:Exercise36State}

impl Exercise36Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_36_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+36)).sum()
}

fn main(){
    println!("Exercise 36: simple_stack");
    let records=[
        Exercise36Record{id:1,label:"Ada".into(),score:98,state:Exercise36State::Finished},
        Exercise36Record{id:2,label:"Linus".into(),score:86,state:Exercise36State::Running},
        Exercise36Record{id:3,label:"Grace".into(),score:73,state:Exercise36State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_36_operation(&[12,7,19,3,25]));
}
