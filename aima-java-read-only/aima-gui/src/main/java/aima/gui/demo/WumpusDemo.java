package aima.gui.demo;

import aima.core.agent.*;
import aima.core.environment.wumpusworld.WumpusCave;
import aima.core.environment.wumpusworld.*;
import aima.core.logic.propositional.parsing.ast.ComplexSentence;
import aima.core.logic.propositional.parsing.ast.Sentence;
import aima.core.logic.propositional.parsing.ast.Connective;


import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * Created by ecgprc on 11/8/14.
 */
public class WumpusDemo {

    public static void main(String[] args) {

        /*
        *  Init The Environment with the map in the book
        *  Note that we created this environment
        */
        WumpusEnv env = new WumpusEnv();
        HybridWumpusAgent agent = env.getAgent();

        /*
            Z signals that no percepts relevant to the agent can be found at that location
            This string will build the map as given
         */
        String init_map_config = "Z,B,Z,B,S,Z,B,Z,Z,BSG,Z,B,S,Z,B,Z";
        env.init_map(init_map_config);

        System.out.print("Map State");
        System.out.println("---------------------------------");
        env.print_map();
        System.out.println("-----------------End Map----------------");


//        /* Print out the init state of the KB */
//        System.out.println("KB Prior To First Move");
//        System.out.println("---------------------------------");
//
//        for( Sentence str : agent.kb.getSentences()){
//            System.out.println(str);
//        }
//
//        System.out.println("-------------End Kb Prior To First Move---------------");


        Action pos =  agent.execute(new AgentPercept(false, false, false, false, false));

        Pattern p = Pattern.compile("[0-9],[0-9]");
        Matcher m = p.matcher(pos.toString());
        int num_moves = 7;
        String [] coords = {"2,1"};
        boolean did_change = false;

        /* Make the correct number of moves through the world */
        while(num_moves > 0){

//            System.out.print(
//                    new ComplexSentence(agent.kb.newSymbol(agent.kb.BREEZE, 2, 1),
//                            Connective.AND,
//                            new ComplexSentence(
//                                    new ComplexSentence(Connective.NOT, agent.kb.newSymbol(agent.kb.OK_TO_MOVE_INTO, 3, 1)), Connective.AND
//                                    , new ComplexSentence(Connective.NOT, agent.kb.newSymbol(agent.kb.OK_TO_MOVE_INTO, 2, 2)))).toString()
//            );


//                //3,1
//                new ComplexSentence(new ComplexSentence(Connective.NOT, agent.kb.newSymbol(agent.kb.WUMPUS,agent.t,3,1)),Connective.AND,
//                        new ComplexSentence(Connective.NOT, agent.kb.newSymbol(agent.kb.OK_TO_MOVE_INTO,agent.t,3,1)));
//                //2,2
//                new ComplexSentence(new ComplexSentence(Connective.NOT, agent.kb.newSymbol(agent.kb.WUMPUS,agent.t,2,2)),Connective.AND,
//                        new ComplexSentence(Connective.NOT, agent.kb.newSymbol(agent.kb.OK_TO_MOVE_INTO,agent.t,2,2)));


            if(m.find()) {
                 coords = m.group().split(",");
            }

            AgentPercept next =  env.getPerceptForAPos(Integer.parseInt(coords[1]) - 1, Integer.parseInt(coords[0] ) - 1);
            pos = agent.execute(next);


            num_moves--;
            m = p.matcher(pos.toString());
        }

    }
}


