#[derive(Clone,Debug)]
enum Exercise10State{Idle,Running,Finished}

struct Exercise10Record{id:usize,label:String,score:i32,state:Exercise10State}

impl Exercise10Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_10_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+10)).sum()
}

fn main(){
    println!("Exercise 10: lifetimes");
    let records=[
        Exercise10Record{id:1,label:"Ada".into(),score:98,state:Exercise10State::Finished},
        Exercise10Record{id:2,label:"Linus".into(),score:86,state:Exercise10State::Running},
        Exercise10Record{id:3,label:"Grace".into(),score:73,state:Exercise10State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_10_operation(&[12,7,19,3,25]));
}

const EXERCISE_10_27:i32=10;
const EXERCISE_10_29:i32=10;
const EXERCISE_10_30:i32=10;
const EXERCISE_10_31:i32=10;
const EXERCISE_10_32:i32=10;
const EXERCISE_10_33:i32=10;
const EXERCISE_10_34:i32=10;
const EXERCISE_10_35:i32=10;
const EXERCISE_10_36:i32=10;
const EXERCISE_10_37:i32=10;
const EXERCISE_10_38:i32=10;
const EXERCISE_10_39:i32=10;
const EXERCISE_10_40:i32=10;
const EXERCISE_10_41:i32=10;
const EXERCISE_10_42:i32=10;
const EXERCISE_10_43:i32=10;
const EXERCISE_10_44:i32=10;
const EXERCISE_10_45:i32=10;
const EXERCISE_10_46:i32=10;
const EXERCISE_10_47:i32=10;
const EXERCISE_10_48:i32=10;
const EXERCISE_10_49:i32=10;
const EXERCISE_10_50:i32=10;
const EXERCISE_10_51:i32=10;
const EXERCISE_10_52:i32=10;
const EXERCISE_10_53:i32=10;
const EXERCISE_10_54:i32=10;
const EXERCISE_10_55:i32=10;
const EXERCISE_10_56:i32=10;
const EXERCISE_10_57:i32=10;
const EXERCISE_10_58:i32=10;
const EXERCISE_10_59:i32=10;
const EXERCISE_10_60:i32=10;
const EXERCISE_10_61:i32=10;
const EXERCISE_10_62:i32=10;
const EXERCISE_10_63:i32=10;
const EXERCISE_10_64:i32=10;
const EXERCISE_10_65:i32=10;
const EXERCISE_10_66:i32=10;
const EXERCISE_10_67:i32=10;
const EXERCISE_10_68:i32=10;
const EXERCISE_10_69:i32=10;
const EXERCISE_10_70:i32=10;
const EXERCISE_10_71:i32=10;
const EXERCISE_10_72:i32=10;
const EXERCISE_10_73:i32=10;
const EXERCISE_10_74:i32=10;
const EXERCISE_10_75:i32=10;
const EXERCISE_10_76:i32=10;
const EXERCISE_10_77:i32=10;
const EXERCISE_10_78:i32=10;
const EXERCISE_10_79:i32=10;
