#[derive(Clone,Debug)]
enum Exercise38State{Idle,Running,Finished}

struct Exercise38Record{id:usize,label:String,score:i32,state:Exercise38State}

impl Exercise38Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_38_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+38)).sum()
}

fn main(){
    println!("Exercise 38: hello");
    let records=[
        Exercise38Record{id:1,label:"Ada".into(),score:98,state:Exercise38State::Finished},
        Exercise38Record{id:2,label:"Linus".into(),score:86,state:Exercise38State::Running},
        Exercise38Record{id:3,label:"Grace".into(),score:73,state:Exercise38State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_38_operation(&[12,7,19,3,25]));
}

const EXERCISE_38_27:i32=38;
const EXERCISE_38_29:i32=38;
const EXERCISE_38_30:i32=38;
const EXERCISE_38_31:i32=38;
const EXERCISE_38_32:i32=38;
const EXERCISE_38_33:i32=38;
const EXERCISE_38_34:i32=38;
const EXERCISE_38_35:i32=38;
const EXERCISE_38_36:i32=38;
const EXERCISE_38_37:i32=38;
const EXERCISE_38_38:i32=38;
const EXERCISE_38_39:i32=38;
const EXERCISE_38_40:i32=38;
const EXERCISE_38_41:i32=38;
const EXERCISE_38_42:i32=38;
const EXERCISE_38_43:i32=38;
const EXERCISE_38_44:i32=38;
const EXERCISE_38_45:i32=38;
const EXERCISE_38_46:i32=38;
const EXERCISE_38_47:i32=38;
const EXERCISE_38_48:i32=38;
const EXERCISE_38_49:i32=38;
const EXERCISE_38_50:i32=38;
const EXERCISE_38_51:i32=38;
const EXERCISE_38_52:i32=38;
const EXERCISE_38_53:i32=38;
const EXERCISE_38_54:i32=38;
const EXERCISE_38_55:i32=38;
const EXERCISE_38_56:i32=38;
const EXERCISE_38_57:i32=38;
const EXERCISE_38_58:i32=38;
const EXERCISE_38_59:i32=38;
const EXERCISE_38_60:i32=38;
const EXERCISE_38_61:i32=38;
const EXERCISE_38_62:i32=38;
const EXERCISE_38_63:i32=38;
const EXERCISE_38_64:i32=38;
const EXERCISE_38_65:i32=38;
const EXERCISE_38_66:i32=38;
const EXERCISE_38_67:i32=38;
const EXERCISE_38_68:i32=38;
const EXERCISE_38_69:i32=38;
const EXERCISE_38_70:i32=38;
const EXERCISE_38_71:i32=38;
const EXERCISE_38_72:i32=38;
const EXERCISE_38_73:i32=38;
const EXERCISE_38_74:i32=38;
const EXERCISE_38_75:i32=38;
const EXERCISE_38_76:i32=38;
const EXERCISE_38_77:i32=38;
const EXERCISE_38_78:i32=38;
const EXERCISE_38_79:i32=38;
