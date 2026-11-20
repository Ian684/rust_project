#[derive(Clone,Debug)]
enum Exercise18State{Idle,Running,Finished}

struct Exercise18Record{id:usize,label:String,score:i32,state:Exercise18State}

impl Exercise18Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_18_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+18)).sum()
}

fn main(){
    println!("Exercise 18: refcell");
    let records=[
        Exercise18Record{id:1,label:"Ada".into(),score:98,state:Exercise18State::Finished},
        Exercise18Record{id:2,label:"Linus".into(),score:86,state:Exercise18State::Running},
        Exercise18Record{id:3,label:"Grace".into(),score:73,state:Exercise18State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_18_operation(&[12,7,19,3,25]));
}

const EXERCISE_18_27:i32=18;
const EXERCISE_18_29:i32=18;
const EXERCISE_18_30:i32=18;
const EXERCISE_18_31:i32=18;
const EXERCISE_18_32:i32=18;
const EXERCISE_18_33:i32=18;
const EXERCISE_18_34:i32=18;
const EXERCISE_18_35:i32=18;
const EXERCISE_18_36:i32=18;
const EXERCISE_18_37:i32=18;
const EXERCISE_18_38:i32=18;
const EXERCISE_18_39:i32=18;
const EXERCISE_18_40:i32=18;
const EXERCISE_18_41:i32=18;
const EXERCISE_18_42:i32=18;
const EXERCISE_18_43:i32=18;
const EXERCISE_18_44:i32=18;
const EXERCISE_18_45:i32=18;
const EXERCISE_18_46:i32=18;
const EXERCISE_18_47:i32=18;
const EXERCISE_18_48:i32=18;
const EXERCISE_18_49:i32=18;
const EXERCISE_18_50:i32=18;
const EXERCISE_18_51:i32=18;
const EXERCISE_18_52:i32=18;
const EXERCISE_18_53:i32=18;
const EXERCISE_18_54:i32=18;
const EXERCISE_18_55:i32=18;
const EXERCISE_18_56:i32=18;
const EXERCISE_18_57:i32=18;
const EXERCISE_18_58:i32=18;
const EXERCISE_18_59:i32=18;
const EXERCISE_18_60:i32=18;
const EXERCISE_18_61:i32=18;
const EXERCISE_18_62:i32=18;
const EXERCISE_18_63:i32=18;
const EXERCISE_18_64:i32=18;
const EXERCISE_18_65:i32=18;
const EXERCISE_18_66:i32=18;
const EXERCISE_18_67:i32=18;
const EXERCISE_18_68:i32=18;
const EXERCISE_18_69:i32=18;
const EXERCISE_18_70:i32=18;
const EXERCISE_18_71:i32=18;
const EXERCISE_18_72:i32=18;
const EXERCISE_18_73:i32=18;
const EXERCISE_18_74:i32=18;
const EXERCISE_18_75:i32=18;
const EXERCISE_18_76:i32=18;
const EXERCISE_18_77:i32=18;
const EXERCISE_18_78:i32=18;
const EXERCISE_18_79:i32=18;
