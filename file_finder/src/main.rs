#[derive(Clone,Debug)]
enum Exercise35State{Idle,Running,Finished}

struct Exercise35Record{id:usize,label:String,score:i32,state:Exercise35State}

impl Exercise35Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_35_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+35)).sum()
}

fn main(){
    println!("Exercise 35: file_finder");
    let records=[
        Exercise35Record{id:1,label:"Ada".into(),score:98,state:Exercise35State::Finished},
        Exercise35Record{id:2,label:"Linus".into(),score:86,state:Exercise35State::Running},
        Exercise35Record{id:3,label:"Grace".into(),score:73,state:Exercise35State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_35_operation(&[12,7,19,3,25]));
}
