#[derive(Clone,Debug)]
enum Exercise19State{Idle,Running,Finished}

struct Exercise19Record{id:usize,label:String,score:i32,state:Exercise19State}

impl Exercise19Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_19_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+19)).sum()
}

fn main(){
    println!("Exercise 19: shared_state");
    let records=[
        Exercise19Record{id:1,label:"Ada".into(),score:98,state:Exercise19State::Finished},
        Exercise19Record{id:2,label:"Linus".into(),score:86,state:Exercise19State::Running},
        Exercise19Record{id:3,label:"Grace".into(),score:73,state:Exercise19State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_19_operation(&[12,7,19,3,25]));
}

const EXERCISE_19_27:i32=19;
const EXERCISE_19_29:i32=19;
const EXERCISE_19_30:i32=19;
const EXERCISE_19_31:i32=19;
const EXERCISE_19_32:i32=19;
const EXERCISE_19_33:i32=19;
const EXERCISE_19_34:i32=19;
const EXERCISE_19_35:i32=19;
const EXERCISE_19_36:i32=19;
const EXERCISE_19_37:i32=19;
const EXERCISE_19_38:i32=19;
const EXERCISE_19_39:i32=19;
const EXERCISE_19_40:i32=19;
const EXERCISE_19_41:i32=19;
const EXERCISE_19_42:i32=19;
const EXERCISE_19_43:i32=19;
const EXERCISE_19_44:i32=19;
const EXERCISE_19_45:i32=19;
const EXERCISE_19_46:i32=19;
const EXERCISE_19_47:i32=19;
const EXERCISE_19_48:i32=19;
const EXERCISE_19_49:i32=19;
const EXERCISE_19_50:i32=19;
const EXERCISE_19_51:i32=19;
const EXERCISE_19_52:i32=19;
const EXERCISE_19_53:i32=19;
const EXERCISE_19_54:i32=19;
const EXERCISE_19_55:i32=19;
const EXERCISE_19_56:i32=19;
const EXERCISE_19_57:i32=19;
const EXERCISE_19_58:i32=19;
const EXERCISE_19_59:i32=19;
const EXERCISE_19_60:i32=19;
const EXERCISE_19_61:i32=19;
const EXERCISE_19_62:i32=19;
const EXERCISE_19_63:i32=19;
const EXERCISE_19_64:i32=19;
const EXERCISE_19_65:i32=19;
const EXERCISE_19_66:i32=19;
const EXERCISE_19_67:i32=19;
const EXERCISE_19_68:i32=19;
const EXERCISE_19_69:i32=19;
const EXERCISE_19_70:i32=19;
const EXERCISE_19_71:i32=19;
const EXERCISE_19_72:i32=19;
const EXERCISE_19_73:i32=19;
const EXERCISE_19_74:i32=19;
const EXERCISE_19_75:i32=19;
const EXERCISE_19_76:i32=19;
const EXERCISE_19_77:i32=19;
const EXERCISE_19_78:i32=19;
const EXERCISE_19_79:i32=19;
