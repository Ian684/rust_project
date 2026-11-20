#[derive(Clone,Debug)]
enum Exercise08State{Idle,Running,Finished}

struct Exercise08Record{id:usize,label:String,score:i32,state:Exercise08State}

impl Exercise08Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_08_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+8)).sum()
}

fn main(){
    println!("Exercise 08: hash_sets");
    let records=[
        Exercise08Record{id:1,label:"Ada".into(),score:98,state:Exercise08State::Finished},
        Exercise08Record{id:2,label:"Linus".into(),score:86,state:Exercise08State::Running},
        Exercise08Record{id:3,label:"Grace".into(),score:73,state:Exercise08State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_08_operation(&[12,7,19,3,25]));
}
