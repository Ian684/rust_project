#[derive(Clone,Debug)]
enum Exercise28State{Idle,Running,Finished}

struct Exercise28Record{id:usize,label:String,score:i32,state:Exercise28State}

impl Exercise28Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_28_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+28)).sum()
}

fn main(){
    println!("Exercise 28: generics");
    let records=[
        Exercise28Record{id:1,label:"Ada".into(),score:98,state:Exercise28State::Finished},
        Exercise28Record{id:2,label:"Linus".into(),score:86,state:Exercise28State::Running},
        Exercise28Record{id:3,label:"Grace".into(),score:73,state:Exercise28State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_28_operation(&[12,7,19,3,25]));
}

const EXERCISE_28_27:i32=28;
const EXERCISE_28_29:i32=28;
const EXERCISE_28_30:i32=28;
const EXERCISE_28_31:i32=28;
const EXERCISE_28_32:i32=28;
const EXERCISE_28_33:i32=28;
const EXERCISE_28_34:i32=28;
const EXERCISE_28_35:i32=28;
const EXERCISE_28_36:i32=28;
const EXERCISE_28_37:i32=28;
const EXERCISE_28_38:i32=28;
const EXERCISE_28_39:i32=28;
const EXERCISE_28_40:i32=28;
const EXERCISE_28_41:i32=28;
const EXERCISE_28_42:i32=28;
const EXERCISE_28_43:i32=28;
const EXERCISE_28_44:i32=28;
const EXERCISE_28_45:i32=28;
const EXERCISE_28_46:i32=28;
const EXERCISE_28_47:i32=28;
const EXERCISE_28_48:i32=28;
const EXERCISE_28_49:i32=28;
const EXERCISE_28_50:i32=28;
const EXERCISE_28_51:i32=28;
const EXERCISE_28_52:i32=28;
const EXERCISE_28_53:i32=28;
const EXERCISE_28_54:i32=28;
const EXERCISE_28_55:i32=28;
const EXERCISE_28_56:i32=28;
const EXERCISE_28_57:i32=28;
const EXERCISE_28_58:i32=28;
const EXERCISE_28_59:i32=28;
const EXERCISE_28_60:i32=28;
const EXERCISE_28_61:i32=28;
const EXERCISE_28_62:i32=28;
const EXERCISE_28_63:i32=28;
const EXERCISE_28_64:i32=28;
const EXERCISE_28_65:i32=28;
const EXERCISE_28_66:i32=28;
const EXERCISE_28_67:i32=28;
const EXERCISE_28_68:i32=28;
const EXERCISE_28_69:i32=28;
const EXERCISE_28_70:i32=28;
const EXERCISE_28_71:i32=28;
const EXERCISE_28_72:i32=28;
const EXERCISE_28_73:i32=28;
const EXERCISE_28_74:i32=28;
const EXERCISE_28_75:i32=28;
const EXERCISE_28_76:i32=28;
const EXERCISE_28_77:i32=28;
const EXERCISE_28_78:i32=28;
const EXERCISE_28_79:i32=28;
