#[derive(Clone,Debug)]
enum Exercise37State{Idle,Running,Finished}

struct Exercise37Record{id:usize,label:String,score:i32,state:Exercise37State}

impl Exercise37Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_37_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+37)).sum()
}

fn main(){
    println!("Exercise 37: safe_split_str");
    let records=[
        Exercise37Record{id:1,label:"Ada".into(),score:98,state:Exercise37State::Finished},
        Exercise37Record{id:2,label:"Linus".into(),score:86,state:Exercise37State::Running},
        Exercise37Record{id:3,label:"Grace".into(),score:73,state:Exercise37State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_37_operation(&[12,7,19,3,25]));
}
