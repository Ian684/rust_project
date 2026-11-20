#[derive(Clone,Debug)]
enum Exercise30State{Idle,Running,Finished}

struct Exercise30Record{id:usize,label:String,score:i32,state:Exercise30State}

impl Exercise30Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_30_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+30)).sum()
}

fn main(){
    println!("Exercise 30: bincode");
    let records=[
        Exercise30Record{id:1,label:"Ada".into(),score:98,state:Exercise30State::Finished},
        Exercise30Record{id:2,label:"Linus".into(),score:86,state:Exercise30State::Running},
        Exercise30Record{id:3,label:"Grace".into(),score:73,state:Exercise30State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_30_operation(&[12,7,19,3,25]));
}
