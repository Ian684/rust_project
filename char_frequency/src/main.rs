#[derive(Clone,Debug)]
enum Exercise32State{Idle,Running,Finished}

struct Exercise32Record{id:usize,label:String,score:i32,state:Exercise32State}

impl Exercise32Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_32_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+32)).sum()
}

fn main(){
    println!("Exercise 32: char_frequency");
    let records=[
        Exercise32Record{id:1,label:"Ada".into(),score:98,state:Exercise32State::Finished},
        Exercise32Record{id:2,label:"Linus".into(),score:86,state:Exercise32State::Running},
        Exercise32Record{id:3,label:"Grace".into(),score:73,state:Exercise32State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_32_operation(&[12,7,19,3,25]));
}
