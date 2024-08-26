'use client';

import React, {useEffect, useState} from 'react';
import {Project} from "@/app/api/models/project";
import {ProjectState} from "@/app/api/models/project-state";
import {useParams} from "next/navigation";
import NavBar from "@/app/components/NavBar";
import {ProjectGroup} from "@/app/api/models/project-group";
import GroupComponent from "@/app/components/data/GroupComponent";
import {showModal} from "@/app/utils";
import {FaPlus} from "react-icons/fa";
import NewGroupModal from "@/app/components/modals/NewGroupModal";

async function getProject(project_id: string): Promise<Project> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/projects/${project_id}`, {
        headers: {"Content-Type": "application/json"},
        credentials: "include",
    });
    const data = await response.json();
    return data as Project;
}

async function getGroupsFromProject(project_id: string): Promise<ProjectGroup[]> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/groups/project/${project_id}`, {
        headers: {"Content-Type": "application/json"},
        credentials: "include",
    });
    const data = await response.json();
    // console.log(data);
    return data.groups as ProjectGroup[];
}

const ProjectDetails = ({project}: { project: Project }) => {
    return (
        <div>
            <p>{project.name}</p>
            <p>{project.description}</p>
            <p>{project.start_date}</p>
            <p>{project.end_date}</p>
            <p>{project.notation_period_duration}</p>
            <p>{project.state}</p>
        </div>
    )
}

interface ProjectDetailsProps {
    project_id: string;
    project: Project;
    setProject: React.Dispatch<React.SetStateAction<Project>>;
}

const ProjectDetailsComponent: React.FC<ProjectDetailsProps> = ({project_id, project, setProject}) => {

    const [projectLoading, setProjectLoading] = useState(true);
    const [projectError, setProjectError] = useState<string | null>(null);

    useEffect(() => {
        const fetchProject = async () => {
            try {
                setProjectLoading(true);
                const promotionData = await getProject(project_id);
                setProject(promotionData);
            } catch (err) {
                setProjectError('Error fetching promotion');
            } finally {
                setProjectLoading(false);
            }
        };

        fetchProject();
    }, [project_id]);

    return (
        <div>
            {projectLoading ? (
                <p>Loading promotion...</p>
            ) : projectError ? (
                <p>{projectError}</p>
            ) : (
                <ProjectDetails project={project}/>
            )}
        </div>
    )
}

const ProjectPage: React.FC = () => {
    const {id: project_id} = useParams<{ id: string }>();

    const [groups, setGroups] = useState<ProjectGroup[]>([]);
    const [groupsLoading, setGroupsLoading] = useState(true);
    const [groupsError, setGroupsError] = useState<string | null>(null);

    const [project, setProject] = useState<Project>({
        id: '',
        description: '',
        end_date: "",
        name: "",
        notation_period_duration: 0,
        promotion_id: "",
        start_date: "",
        state: ProjectState.NotStarted
    });

    useEffect(() => {
        const fetchGroups = async () => {
            try {
                setGroupsLoading(true);
                const groupsData = await getGroupsFromProject(project_id);
                console.log("Fetched groups data:", groupsData);
                setGroups(groupsData);
            } catch (err) {
                setGroupsError('Error fetching groups');
            } finally {
                setGroupsLoading(false);
            }
        };

        fetchGroups();
    }, [project_id]);

    useEffect(() => {
        console.log("Updated groups state:", groups);
    }, [groups]);

    return (
        <div>
            <NavBar/>
            <div className="flex-col">
                <ProjectDetailsComponent project_id={project_id} project={project} setProject={setProject}/>
                <div className="flex-col">
                    {groupsLoading ? (
                        <p>Loading groups...</p>
                    ) : groupsError ? (
                        <p>{groupsError}</p>
                    ) : groups && groups.length > 0 ? (
                        groups.map((group) => (
                            <GroupComponent
                                key={group.group.id}
                                info={group}
                                withMarks={true}
                                onDelete={(id: string) => {}}
                             projectState={project.state}/>
                        ))
                    ) : <p>No groups</p>
                    }
                    <NewGroupModal groups={groups} setGroups={setGroups} project_id={project_id} key={project_id} />
                    <button onClick={() => showModal("new_group_modal")}>
                        <FaPlus className="size-10"/>
                    </button>
                </div>
            </div>
        </div>
    )
}


export default ProjectPage;