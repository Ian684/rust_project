#[derive(Clone,Debug)]
enum Exercise01State{Idle,Running,Finished}

struct Exercise01Record{id:usize,label:String,score:i32,state:Exercise01State}

impl Exercise01Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_01_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+1)).sum()
}

fn main(){
    println!("Exercise 01: arc_mutex");
    let records=[
        Exercise01Record{id:1,label:"Ada".into(),score:98,state:Exercise01State::Finished},
        Exercise01Record{id:2,label:"Linus".into(),score:86,state:Exercise01State::Running},
        Exercise01Record{id:3,label:"Grace".into(),score:73,state:Exercise01State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_01_operation(&[12,7,19,3,25]));
}

const EXERCISE_01_27:i32=1;
const EXERCISE_01_29:i32=1;
const EXERCISE_01_30:i32=1;
const EXERCISE_01_31:i32=1;
const EXERCISE_01_32:i32=1;
const EXERCISE_01_33:i32=1;
const EXERCISE_01_34:i32=1;
const EXERCISE_01_35:i32=1;
const EXERCISE_01_36:i32=1;
const EXERCISE_01_37:i32=1;
const EXERCISE_01_38:i32=1;
const EXERCISE_01_39:i32=1;
const EXERCISE_01_40:i32=1;
const EXERCISE_01_41:i32=1;
const EXERCISE_01_42:i32=1;
const EXERCISE_01_43:i32=1;
const EXERCISE_01_44:i32=1;
const EXERCISE_01_45:i32=1;
const EXERCISE_01_46:i32=1;
const EXERCISE_01_47:i32=1;
const EXERCISE_01_48:i32=1;
const EXERCISE_01_49:i32=1;
const EXERCISE_01_50:i32=1;
const EXERCISE_01_51:i32=1;
const EXERCISE_01_52:i32=1;
const EXERCISE_01_53:i32=1;
const EXERCISE_01_54:i32=1;
const EXERCISE_01_55:i32=1;
const EXERCISE_01_56:i32=1;
const EXERCISE_01_57:i32=1;
const EXERCISE_01_58:i32=1;
const EXERCISE_01_59:i32=1;
const EXERCISE_01_60:i32=1;
const EXERCISE_01_61:i32=1;
const EXERCISE_01_62:i32=1;
const EXERCISE_01_63:i32=1;
const EXERCISE_01_64:i32=1;
const EXERCISE_01_65:i32=1;
const EXERCISE_01_66:i32=1;
const EXERCISE_01_67:i32=1;
const EXERCISE_01_68:i32=1;
const EXERCISE_01_69:i32=1;
const EXERCISE_01_70:i32=1;
const EXERCISE_01_71:i32=1;
const EXERCISE_01_72:i32=1;
const EXERCISE_01_73:i32=1;
const EXERCISE_01_74:i32=1;
const EXERCISE_01_75:i32=1;
const EXERCISE_01_76:i32=1;
const EXERCISE_01_77:i32=1;
const EXERCISE_01_78:i32=1;
const EXERCISE_01_79:i32=1;
