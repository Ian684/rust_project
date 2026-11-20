#[derive(Clone,Debug)]
enum Exercise17State{Idle,Running,Finished}

struct Exercise17Record{id:usize,label:String,score:i32,state:Exercise17State}

impl Exercise17Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_17_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+17)).sum()
}

fn main(){
    println!("Exercise 17: read_input");
    let records=[
        Exercise17Record{id:1,label:"Ada".into(),score:98,state:Exercise17State::Finished},
        Exercise17Record{id:2,label:"Linus".into(),score:86,state:Exercise17State::Running},
        Exercise17Record{id:3,label:"Grace".into(),score:73,state:Exercise17State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_17_operation(&[12,7,19,3,25]));
}
