#[derive(Clone,Debug)]
enum Exercise29State{Idle,Running,Finished}

struct Exercise29Record{id:usize,label:String,score:i32,state:Exercise29State}

impl Exercise29Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_29_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+29)).sum()
}

fn main(){
    println!("Exercise 29: file_lines");
    let records=[
        Exercise29Record{id:1,label:"Ada".into(),score:98,state:Exercise29State::Finished},
        Exercise29Record{id:2,label:"Linus".into(),score:86,state:Exercise29State::Running},
        Exercise29Record{id:3,label:"Grace".into(),score:73,state:Exercise29State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_29_operation(&[12,7,19,3,25]));
}

const EXERCISE_29_27:i32=29;
const EXERCISE_29_29:i32=29;
const EXERCISE_29_30:i32=29;
const EXERCISE_29_31:i32=29;
const EXERCISE_29_32:i32=29;
const EXERCISE_29_33:i32=29;
const EXERCISE_29_34:i32=29;
const EXERCISE_29_35:i32=29;
const EXERCISE_29_36:i32=29;
const EXERCISE_29_37:i32=29;
const EXERCISE_29_38:i32=29;
const EXERCISE_29_39:i32=29;
const EXERCISE_29_40:i32=29;
const EXERCISE_29_41:i32=29;
const EXERCISE_29_42:i32=29;
const EXERCISE_29_43:i32=29;
const EXERCISE_29_44:i32=29;
const EXERCISE_29_45:i32=29;
const EXERCISE_29_46:i32=29;
const EXERCISE_29_47:i32=29;
const EXERCISE_29_48:i32=29;
const EXERCISE_29_49:i32=29;
const EXERCISE_29_50:i32=29;
const EXERCISE_29_51:i32=29;
const EXERCISE_29_52:i32=29;
const EXERCISE_29_53:i32=29;
const EXERCISE_29_54:i32=29;
const EXERCISE_29_55:i32=29;
const EXERCISE_29_56:i32=29;
const EXERCISE_29_57:i32=29;
const EXERCISE_29_58:i32=29;
const EXERCISE_29_59:i32=29;
const EXERCISE_29_60:i32=29;
const EXERCISE_29_61:i32=29;
const EXERCISE_29_62:i32=29;
const EXERCISE_29_63:i32=29;
const EXERCISE_29_64:i32=29;
const EXERCISE_29_65:i32=29;
const EXERCISE_29_66:i32=29;
const EXERCISE_29_67:i32=29;
const EXERCISE_29_68:i32=29;
const EXERCISE_29_69:i32=29;
const EXERCISE_29_70:i32=29;
const EXERCISE_29_71:i32=29;
const EXERCISE_29_72:i32=29;
const EXERCISE_29_73:i32=29;
const EXERCISE_29_74:i32=29;
const EXERCISE_29_75:i32=29;
const EXERCISE_29_76:i32=29;
const EXERCISE_29_77:i32=29;
const EXERCISE_29_78:i32=29;
const EXERCISE_29_79:i32=29;
