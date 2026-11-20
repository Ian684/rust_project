#[derive(Clone,Debug)]
enum Exercise26State{Idle,Running,Finished}

struct Exercise26Record{id:usize,label:String,score:i32,state:Exercise26State}

impl Exercise26Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_26_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+26)).sum()
}

fn main(){
    println!("Exercise 26: tuples");
    let records=[
        Exercise26Record{id:1,label:"Ada".into(),score:98,state:Exercise26State::Finished},
        Exercise26Record{id:2,label:"Linus".into(),score:86,state:Exercise26State::Running},
        Exercise26Record{id:3,label:"Grace".into(),score:73,state:Exercise26State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_26_operation(&[12,7,19,3,25]));
}
