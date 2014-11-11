package aima.gui.demo;

import aima.core.agent.*;
import aima.core.environment.wumpusworld.WumpusCave;
import aima.core.environment.wumpusworld.*;
import aima.core.logic.propositional.parsing.ast.Sentence;

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


        /* Print out the init state of the KB */
        System.out.println("KB Prior To First Move");
        System.out.println("---------------------------------");

        for( Sentence str : agent.kb.getSentences()){
            System.out.println(str);
        }

        System.out.println("-------------End Kb Prior To First Move---------------");

        /* Start in 1,1 with the percepts all set to false as in the book */
        System.out.println("--------First Percept and Action Sequence-----------");
        System.out.println("---------------------------------");
        System.out.println(new AgentPercept(false,false,false,false,false).toString());
        Action pos =  agent.execute(new AgentPercept(false, false, false, false, false));
        System.out.println(pos.toString());
        System.out.println("----------End Agent First Percept and Action---------\n\n");



        System.out.println("\n\n---------KB After First Move-----------");
        System.out.println("-----------------------------------");
        for( Sentence str : agent.kb.getSentences()){
            System.out.println(str);
        }
        System.out.println("-------------End Kb After First Move---------------");

        Pattern p = Pattern.compile("[0-9],[0-9]");
        Matcher m = p.matcher(pos.toString());
        int num_moves = 3;
        String [] coords = {"2,1"};
        boolean did_change = false;

        /* Make the correct number of moves through the world */
        while(num_moves > 0){
            if(m.find()) {
                 coords = m.group().split(",");
            }
            System.out.println("\n-----------Next Percept and action ------------");
            AgentPercept next =  env.getPerceptForAPos(Integer.parseInt(coords[0]), Integer.parseInt(coords[1]));

            for( String str : coords ){
               System.out.println("##############");
                System.out.println(str);
                System.out.println("##############");

            }

            pos = agent.execute(next);
            System.out.println(next.toString());
            System.out.println(pos.toString());
            System.out.println("-----------End Next Percept and action --------\n");

            System.out.println("-----------Next Associated KB (percept above) ------------");

            for( Sentence str : agent.kb.getSentences()){
                System.out.println(str);
            }

            System.out.println("-----------End Associated KB ------------------");
            num_moves--;
            m = p.matcher(pos.toString());
        }

    }
}


